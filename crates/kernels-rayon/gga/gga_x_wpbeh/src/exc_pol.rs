//! GGA_X_WPBEH exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wpbeh.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_wpbeh_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    // Loop-invariant bindings (constants, parameters, thresholds):
    // the same statements maple2c emits per point, evaluated once.
    let t3 = f64x8::splat(M_CBRT3);
    let t4 = f64x8::splat(M_CBRTPI);
    let t6 = t3 / t4;
    let t13 = zeta_threshold - f64x8::splat(1.0);
    let t17 = -t13;
    let t23 = (simd::cbrt(zeta_threshold));
    let t24 = t23 * zeta_threshold;
    let t30 = t3 * t3;
    let t31 = param_hyb_omega_0 * t30;
    let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
    let t33 = f64x8::splat(2.1450293971110255);
    let t34 = f64x8::splat(1.0) / t33;
    let t51 = f64x8::splat(M_CBRT6);
    let t52 = t51 * t51;
    let t53 = t52 * t34;
    let t92 = param_hyb_omega_0 * param_hyb_omega_0;
    let t95 = t33 * t33;
    let t96 = f64x8::splat(1.0) / t95;
    let t249 = f64x8::splat(3.0) * f64x8::splat(0.25) * f64x8::splat(M_PI);
    let t250 = f64x8::splat(1.7724538509055159);
    let t273 = f64x8::splat(1.0) / t250;
    let t290 = t92 * t3;
    let t314 = t92 * param_hyb_omega_0 / t32;
    let t328 = t92 * t92;
    let t330 = t328 * param_hyb_omega_0 * t3;
    let t332 = f64x8::splat(1.0) / t95 / t32;
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t12 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t16 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t18 = v_rho0 - v_rho1;
            let t19 = t18 * t8;
            let t20 = ((t12).select(t13, (t16).select(t17, t19)));
            let t21 = f64x8::splat(1.0) + t20;
            let t22 = (t21).simd_le(zeta_threshold);
            let t25 = (simd::cbrt(t21));
            let t27 = ((t22).select(t24, t25 * t21));
            let t28 = (simd::cbrt(t7));
            let t29 = t27 * t28;
            let t36 = (f64x8::splat(1.0) + t19).simd_le(zeta_threshold);
            let t38 = (f64x8::splat(1.0) - t19).simd_le(zeta_threshold);
            let t39 = ((t36).select(t13, (t38).select(t17, t19)));
            let t40 = f64x8::splat(1.0) + t39;
            let t41 = (t40).simd_le(zeta_threshold);
            let t42 = (simd::cbrt(t40));
            let t43 = ((t41).select(t23, t42));
            let t44 = f64x8::splat(1.0) / t43;
            let t45 = t34 * t44;
            let t46 = f64x8::splat(1.0) / t28;
            let t48 = t31 * t45 * t46;
            let t49 = t48 / f64x8::splat(3.0);
            let t50 = (f64x8::splat(14.0)).simd_lt(t49);
            let t54 = ((v_sigma0).sqrt());
            let t55 = (simd::cbrt(v_rho0));
            let t57 = f64x8::splat(1.0) / t55 / v_rho0;
            let t60 = t53 * t54 * t57 / f64x8::splat(12.0);
            let t61 = (t60).simd_lt(f64x8::splat(1.0));
            let t62 = (f64x8::splat(15.0)).simd_lt(t60);
            let t63 = ((t62).select(f64x8::splat(15.0), t60));
            let t64 = (f64x8::splat(1.0)).simd_lt(t63);
            let t65 = ((t64).select(t63, f64x8::splat(1.0)));
            let t67 = (simd::exp(t65 - f64x8::splat(8.572844)));
            let t68 = f64x8::splat(1.0) + t67;
            let t69 = (simd::ln(t68));
            let t71 = ((t62).select(f64x8::splat(8.572844), t65 - t69));
            let t72 = ((t61).select(t60, t71));
            let t73 = (t72).simd_lt(f64x8::splat(1e-15));
            let t74 = ((t73).select(f64x8::splat(1e-15), t72));
            let t75 = t74 * t74;
            let t77 = t75 * t75;
            let t79 = f64x8::splat(0.00979681) * t75 + f64x8::splat(0.0410834) * t77;
            let t80 = t75 * t79;
            let t82 = t77 * t74;
            let t84 = t77 * t75;
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.18744) * t77 + f64x8::splat(0.00120824) * t82 + f64x8::splat(0.0347188) * t84;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t80 * t87;
            let t89 = f64x8::splat(2.214317600459161) * t88;
            let t90 = (t49).simd_lt(f64x8::splat(14.0));
            let t91 = ((t90).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t93 = t91 * t92;
            let t94 = t93 * t3;
            let t97 = t43 * t43;
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t96 * t98;
            let t100 = t28 * t28;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t99 * t101;
            let t103 = t94 * t102;
            let t105 = t89 + f64x8::splat(0.7381058668197203) * t103;
            let t106 = (simd::e1_scaled(t105));
            let t108 = t103 / f64x8::splat(3.0);
            let t109 = f64x8::splat(0.57786348) + t88 + t108;
            let t110 = (simd::ln(t109));
            let t112 = t88 + t108;
            let t113 = (simd::ln(t112));
            let t116 = ((t50).select(f64x8::splat(14.0), t49));
            let t118 = t116 * t116;
            let t119 = t118 * t116;
            let t121 = t118 * t118;
            let t122 = t121 * t116;
            let t124 = t121 * t119;
            let t127 = (f64x8::splat(1.7059169152930056) * t116 - f64x8::splat(4.162270540644039) * t119 + f64x8::splat(4.217437034869465) * t122 - f64x8::splat(1.0676080470633098) * t124) * f64x8::splat(M_PI);
            let t128 = (t116).simd_lt(f64x8::splat(14.0));
            let t129 = ((t128).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t130 = t129 * t118;
            let t132 = t89 + f64x8::splat(2.214317600459161) * t130;
            let t133 = ((t132).sqrt());
            let t134 = (simd::erfcx(t133));
            let t139 = t121 * t118;
            let t141 = t121 * t121;
            let t143 = -f64x8::splat(1.0161144) + f64x8::splat(3.2686565979666846) * t118 - f64x8::splat(4.841839888141759) * t121 + f64x8::splat(2.723636568586566) * t139 - f64x8::splat(0.20524577845574896) * t141;
            let t144 = (simd::e1_scaled(t132));
            let t147 = f64x8::splat(0.57786348) + t88 + t130;
            let t148 = ((t147).sqrt());
            let t149 = f64x8::splat(1.0) / t148;
            let t152 = f64x8::splat(1.0) / t147;
            let t155 = t88 + t130;
            let t156 = ((t155).sqrt());
            let t157 = f64x8::splat(1.0) / t156;
            let t159 = t148 * t147;
            let t160 = f64x8::splat(1.0) / t159;
            let t162 = f64x8::splat(2.478878780461809) * t157 - f64x8::splat(0.5597387610403739) * t160;
            let t164 = f64x8::splat(1.0) / t155;
            let t166 = t147 * t147;
            let t167 = f64x8::splat(1.0) / t166;
            let t169 = -f64x8::splat(1.0933029406300512) * t164 + f64x8::splat(0.49374260512735113) * t167;
            let t171 = t148 * t166;
            let t174 = f64x8::splat(9.0) * t88 + f64x8::splat(9.0) * t130 - f64x8::splat(2.0322288);
            let t177 = t156 * t155;
            let t179 = f64x8::splat(3.0) * t171 * t174 + f64x8::splat(4.12995389554944) * t177;
            let t180 = f64x8::splat(1.0) / t171;
            let t181 = t179 * t180;
            let t182 = f64x8::splat(1.0) / t177;
            let t183 = t182 * t122;
            let t186 = t166 * t147;
            let t187 = f64x8::splat(1.0) / t186;
            let t190 = -f64x8::splat(36.0) + f64x8::splat(79.7154336165298) * t88;
            let t191 = t155 * t155;
            let t192 = f64x8::splat(1.0) / t191;
            let t195 = f64x8::splat(0.2508588461882105) * t187 + f64x8::splat(0.007715016088131) * t190 * t192;
            let t197 = t156 * t191;
            let t199 = t148 * t186;
            let t203 = f64x8::splat(27.0) * t191 - f64x8::splat(6.0966864) * t88 - f64x8::splat(6.0966864) * t130 + f64x8::splat(4.12995389554944);
            let t206 = -f64x8::splat(41.96505624603882) * t197 + f64x8::splat(9.0) * t199 * t203;
            let t207 = f64x8::splat(1.0) / t199;
            let t208 = t206 * t207;
            let t209 = f64x8::splat(1.0) / t197;
            let t210 = t209 * t124;
            let t213 = t166 * t166;
            let t214 = t129 * t213;
            let t217 = t191 * t155;
            let t222 = -f64x8::splat(729.0) * t191 + f64x8::splat(329.2210656) * t88 + f64x8::splat(329.2210656) * t130 - f64x8::splat(297.35668047955966);
            let t225 = f64x8::splat(81.27826616498021) * t214 * t155 + f64x8::splat(3.384784484376542) * t217 + f64x8::splat(0.008401793031216) * t213 * t222;
            let t226 = f64x8::splat(1.0) / t213;
            let t227 = t225 * t226;
            let t228 = f64x8::splat(1.0) / t217;
            let t229 = t228 * t141;
            let t233 = (simd::ln(t155 * t152));
            let t235 = t127 * t134 * f64x8::splat(0.5) - t143 * t144 * f64x8::splat(0.5) - f64x8::splat(1.0159746228068032) * t149 * t116 + f64x8::splat(0.738073119521991) * t152 * t118 + t162 * t119 + t169 * t121 - f64x8::splat(0.09302717396924197) * t181 * t183 + t195 * t139 + f64x8::splat(0.0026165591067112575) * t208 * t210 + f64x8::splat(0.007566670425467926) * t227 * t229 + f64x8::splat(0.5080572) * t233;
            let t236 = ((t50).select(f64x8::splat(0.5080572) * t106 - f64x8::splat(0.5080572) * t110 + f64x8::splat(0.5080572) * t113, t235));
            let t238 = f64x8::splat(0.57786348) + t88;
            let t239 = t238 * t238;
            let t241 = f64x8::splat(0.077215461) * t88;
            let t242 = t238 * t75;
            let t245 = f64x8::splat(6.4753871) * t79 * t87 + f64x8::splat(0.4796583);
            let t248 = (f64x8::splat(0.08)).simd_lt(t74);
            let t252 = t245 * t75 + f64x8::splat(1.0);
            let t256 = t239 * t238;
            let t259 = t250 * (-f64x8::splat(0.779335965) - f64x8::splat(0.463292766) * t252 * t238 - f64x8::splat(1.48683344) * t239 + f64x8::splat(8.1289152) * t256);
            let t260 = ((t238).sqrt());
            let t261 = t260 * t256;
            let t262 = f64x8::splat(1.0) / t261;
            let t265 = (simd::exp(t89));
            let t266 = ((t88).sqrt());
            let t268 = (simd::erf(f64x8::splat(1.4880583323442536) * t266));
            let t269 = f64x8::splat(1.0) - t268;
            let t270 = t265 * t269;
            let t274 = (t249 + t259 * t262 * f64x8::splat(0.0625) - f64x8::splat(2.3751029502456897) * t270) * t273;
            let t275 = f64x8::splat(1.0) / t75;
            let t276 = t275 * t261;
            let t282 = ((t248).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t274 * t276, -f64x8::splat(0.0262841788) - f64x8::splat(0.07117647788) * t75 + f64x8::splat(0.08534541323) * t77));
            let t283 = t75 * t282;
            let t285 = -f64x8::splat(0.37170836) * t239 - f64x8::splat(0.14853145700326428) - t241 - f64x8::splat(0.077215461) * t242 * t245 + f64x8::splat(2.0) * t283;
            let t286 = f64x8::splat(1.0) / t256;
            let t289 = t31 * t45;
            let t291 = t290 * t102;
            let t293 = f64x8::splat(0.57786348) + t88 + t291 / f64x8::splat(3.0);
            let t294 = t293 * t293;
            let t298 = t293 * t75;
            let t299 = t298 * t245;
            let t302 = -f64x8::splat(1.48683344) * t294 - f64x8::splat(1.0470559350195856) - f64x8::splat(0.463292766) * t88 - f64x8::splat(0.154430922) * t291 - f64x8::splat(0.463292766) * t299 + f64x8::splat(15.0) * t283;
            let t303 = t46 * t302;
            let t304 = f64x8::splat(1.0) / t238;
            let t305 = ((t293).sqrt());
            let t307 = f64x8::splat(1.0) / t305 / t294;
            let t308 = t304 * t307;
            let t315 = t97 * t43;
            let t316 = f64x8::splat(1.0) / t315;
            let t317 = t314 * t316;
            let t321 = -f64x8::splat(0.30439865000326427) - t241 - f64x8::splat(0.025738487) * t291 - f64x8::splat(0.077215461) * t299 + f64x8::splat(5.0) * t283;
            let t323 = f64x8::splat(1.0) / t239;
            let t324 = t323 * t307;
            let t333 = t97 * t97;
            let t335 = f64x8::splat(1.0) / t333 / t43;
            let t336 = t332 * t335;
            let t337 = t330 * t336;
            let t339 = f64x8::splat(1.0) / t100 / t7;
            let t340 = -f64x8::splat(0.051955731) + t283;
            let t341 = t339 * t340;
            let t342 = t286 * t307;
            let t346 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t236 - f64x8::splat(4.0) / f64x8::splat(9.0) * t285 * t286 + t289 * t303 * t308 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t317 * t8 * t321 * t324 + f64x8::splat(8.0) / f64x8::splat(81.0) * t337 * t341 * t342;
            let t350 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) * f64x8::splat(0.125) * t6 * t29 * t346));
            let t351 = (v_rho1).simd_le(dens_threshold);
            let t352 = -t18;
            let t354 = ((t16).select(t13, (t12).select(t17, t352 * t8)));
            let t355 = f64x8::splat(1.0) + t354;
            let t356 = (t355).simd_le(zeta_threshold);
            let t357 = (simd::cbrt(t355));
            let t359 = ((t356).select(t24, t357 * t355));
            let t360 = t359 * t28;
            let t361 = ((t38).select(t13, (t36).select(t17, -t19)));
            let t362 = f64x8::splat(1.0) + t361;
            let t363 = (t362).simd_le(zeta_threshold);
            let t364 = (simd::cbrt(t362));
            let t365 = ((t363).select(t23, t364));
            let t366 = f64x8::splat(1.0) / t365;
            let t367 = t34 * t366;
            let t369 = t31 * t367 * t46;
            let t370 = t369 / f64x8::splat(3.0);
            let t371 = (f64x8::splat(14.0)).simd_lt(t370);
            let t372 = ((v_sigma2).sqrt());
            let t373 = (simd::cbrt(v_rho1));
            let t375 = f64x8::splat(1.0) / t373 / v_rho1;
            let t378 = t53 * t372 * t375 / f64x8::splat(12.0);
            let t379 = (t378).simd_lt(f64x8::splat(1.0));
            let t380 = (f64x8::splat(15.0)).simd_lt(t378);
            let t381 = ((t380).select(f64x8::splat(15.0), t378));
            let t382 = (f64x8::splat(1.0)).simd_lt(t381);
            let t383 = ((t382).select(t381, f64x8::splat(1.0)));
            let t385 = (simd::exp(t383 - f64x8::splat(8.572844)));
            let t386 = f64x8::splat(1.0) + t385;
            let t387 = (simd::ln(t386));
            let t389 = ((t380).select(f64x8::splat(8.572844), t383 - t387));
            let t390 = ((t379).select(t378, t389));
            let t391 = (t390).simd_lt(f64x8::splat(1e-15));
            let t392 = ((t391).select(f64x8::splat(1e-15), t390));
            let t393 = t392 * t392;
            let t395 = t393 * t393;
            let t397 = f64x8::splat(0.00979681) * t393 + f64x8::splat(0.0410834) * t395;
            let t398 = t393 * t397;
            let t400 = t395 * t392;
            let t402 = t395 * t393;
            let t404 = f64x8::splat(1.0) + f64x8::splat(0.18744) * t395 + f64x8::splat(0.00120824) * t400 + f64x8::splat(0.0347188) * t402;
            let t405 = f64x8::splat(1.0) / t404;
            let t406 = t398 * t405;
            let t407 = f64x8::splat(2.214317600459161) * t406;
            let t408 = (t370).simd_lt(f64x8::splat(14.0));
            let t409 = ((t408).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t410 = t409 * t92;
            let t411 = t410 * t3;
            let t412 = t365 * t365;
            let t413 = f64x8::splat(1.0) / t412;
            let t414 = t96 * t413;
            let t415 = t414 * t101;
            let t416 = t411 * t415;
            let t418 = t407 + f64x8::splat(0.7381058668197203) * t416;
            let t419 = (simd::e1_scaled(t418));
            let t421 = t416 / f64x8::splat(3.0);
            let t422 = f64x8::splat(0.57786348) + t406 + t421;
            let t423 = (simd::ln(t422));
            let t425 = t406 + t421;
            let t426 = (simd::ln(t425));
            let t429 = ((t371).select(f64x8::splat(14.0), t370));
            let t431 = t429 * t429;
            let t432 = t431 * t429;
            let t434 = t431 * t431;
            let t435 = t434 * t429;
            let t437 = t434 * t432;
            let t440 = (f64x8::splat(1.7059169152930056) * t429 - f64x8::splat(4.162270540644039) * t432 + f64x8::splat(4.217437034869465) * t435 - f64x8::splat(1.0676080470633098) * t437) * f64x8::splat(M_PI);
            let t441 = (t429).simd_lt(f64x8::splat(14.0));
            let t442 = ((t441).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t443 = t442 * t431;
            let t445 = t407 + f64x8::splat(2.214317600459161) * t443;
            let t446 = ((t445).sqrt());
            let t447 = (simd::erfcx(t446));
            let t452 = t434 * t431;
            let t454 = t434 * t434;
            let t456 = -f64x8::splat(1.0161144) + f64x8::splat(3.2686565979666846) * t431 - f64x8::splat(4.841839888141759) * t434 + f64x8::splat(2.723636568586566) * t452 - f64x8::splat(0.20524577845574896) * t454;
            let t457 = (simd::e1_scaled(t445));
            let t460 = f64x8::splat(0.57786348) + t406 + t443;
            let t461 = ((t460).sqrt());
            let t462 = f64x8::splat(1.0) / t461;
            let t465 = f64x8::splat(1.0) / t460;
            let t468 = t406 + t443;
            let t469 = ((t468).sqrt());
            let t470 = f64x8::splat(1.0) / t469;
            let t472 = t461 * t460;
            let t473 = f64x8::splat(1.0) / t472;
            let t475 = f64x8::splat(2.478878780461809) * t470 - f64x8::splat(0.5597387610403739) * t473;
            let t477 = f64x8::splat(1.0) / t468;
            let t479 = t460 * t460;
            let t480 = f64x8::splat(1.0) / t479;
            let t482 = -f64x8::splat(1.0933029406300512) * t477 + f64x8::splat(0.49374260512735113) * t480;
            let t484 = t461 * t479;
            let t487 = f64x8::splat(9.0) * t406 + f64x8::splat(9.0) * t443 - f64x8::splat(2.0322288);
            let t490 = t469 * t468;
            let t492 = f64x8::splat(3.0) * t484 * t487 + f64x8::splat(4.12995389554944) * t490;
            let t493 = f64x8::splat(1.0) / t484;
            let t494 = t492 * t493;
            let t495 = f64x8::splat(1.0) / t490;
            let t496 = t495 * t435;
            let t499 = t479 * t460;
            let t500 = f64x8::splat(1.0) / t499;
            let t503 = -f64x8::splat(36.0) + f64x8::splat(79.7154336165298) * t406;
            let t504 = t468 * t468;
            let t505 = f64x8::splat(1.0) / t504;
            let t508 = f64x8::splat(0.2508588461882105) * t500 + f64x8::splat(0.007715016088131) * t503 * t505;
            let t510 = t469 * t504;
            let t512 = t461 * t499;
            let t516 = f64x8::splat(27.0) * t504 - f64x8::splat(6.0966864) * t406 - f64x8::splat(6.0966864) * t443 + f64x8::splat(4.12995389554944);
            let t519 = -f64x8::splat(41.96505624603882) * t510 + f64x8::splat(9.0) * t512 * t516;
            let t520 = f64x8::splat(1.0) / t512;
            let t521 = t519 * t520;
            let t522 = f64x8::splat(1.0) / t510;
            let t523 = t522 * t437;
            let t526 = t479 * t479;
            let t527 = t442 * t526;
            let t530 = t504 * t468;
            let t535 = -f64x8::splat(729.0) * t504 + f64x8::splat(329.2210656) * t406 + f64x8::splat(329.2210656) * t443 - f64x8::splat(297.35668047955966);
            let t538 = f64x8::splat(81.27826616498021) * t527 * t468 + f64x8::splat(3.384784484376542) * t530 + f64x8::splat(0.008401793031216) * t526 * t535;
            let t539 = f64x8::splat(1.0) / t526;
            let t540 = t538 * t539;
            let t541 = f64x8::splat(1.0) / t530;
            let t542 = t541 * t454;
            let t546 = (simd::ln(t468 * t465));
            let t548 = t440 * t447 * f64x8::splat(0.5) - t456 * t457 * f64x8::splat(0.5) - f64x8::splat(1.0159746228068032) * t462 * t429 + f64x8::splat(0.738073119521991) * t465 * t431 + t475 * t432 + t482 * t434 - f64x8::splat(0.09302717396924197) * t494 * t496 + t508 * t452 + f64x8::splat(0.0026165591067112575) * t521 * t523 + f64x8::splat(0.007566670425467926) * t540 * t542 + f64x8::splat(0.5080572) * t546;
            let t549 = ((t371).select(f64x8::splat(0.5080572) * t419 - f64x8::splat(0.5080572) * t423 + f64x8::splat(0.5080572) * t426, t548));
            let t551 = f64x8::splat(0.57786348) + t406;
            let t552 = t551 * t551;
            let t554 = f64x8::splat(0.077215461) * t406;
            let t555 = t551 * t393;
            let t558 = f64x8::splat(6.4753871) * t397 * t405 + f64x8::splat(0.4796583);
            let t561 = (f64x8::splat(0.08)).simd_lt(t392);
            let t563 = t558 * t393 + f64x8::splat(1.0);
            let t567 = t552 * t551;
            let t570 = t250 * (-f64x8::splat(0.779335965) - f64x8::splat(0.463292766) * t563 * t551 - f64x8::splat(1.48683344) * t552 + f64x8::splat(8.1289152) * t567);
            let t571 = ((t551).sqrt());
            let t572 = t571 * t567;
            let t573 = f64x8::splat(1.0) / t572;
            let t576 = (simd::exp(t407));
            let t577 = ((t406).sqrt());
            let t579 = (simd::erf(f64x8::splat(1.4880583323442536) * t577));
            let t580 = f64x8::splat(1.0) - t579;
            let t581 = t576 * t580;
            let t584 = (t249 + t570 * t573 * f64x8::splat(0.0625) - f64x8::splat(2.3751029502456897) * t581) * t273;
            let t585 = f64x8::splat(1.0) / t393;
            let t586 = t585 * t572;
            let t592 = ((t561).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t584 * t586, -f64x8::splat(0.0262841788) - f64x8::splat(0.07117647788) * t393 + f64x8::splat(0.08534541323) * t395));
            let t593 = t393 * t592;
            let t595 = -f64x8::splat(0.37170836) * t552 - f64x8::splat(0.14853145700326428) - t554 - f64x8::splat(0.077215461) * t555 * t558 + f64x8::splat(2.0) * t593;
            let t596 = f64x8::splat(1.0) / t567;
            let t599 = t31 * t367;
            let t600 = t290 * t415;
            let t602 = f64x8::splat(0.57786348) + t406 + t600 / f64x8::splat(3.0);
            let t603 = t602 * t602;
            let t607 = t602 * t393;
            let t608 = t607 * t558;
            let t611 = -f64x8::splat(1.48683344) * t603 - f64x8::splat(1.0470559350195856) - f64x8::splat(0.463292766) * t406 - f64x8::splat(0.154430922) * t600 - f64x8::splat(0.463292766) * t608 + f64x8::splat(15.0) * t593;
            let t612 = t46 * t611;
            let t613 = f64x8::splat(1.0) / t551;
            let t614 = ((t602).sqrt());
            let t616 = f64x8::splat(1.0) / t614 / t603;
            let t617 = t613 * t616;
            let t621 = t412 * t365;
            let t622 = f64x8::splat(1.0) / t621;
            let t623 = t314 * t622;
            let t627 = -f64x8::splat(0.30439865000326427) - t554 - f64x8::splat(0.025738487) * t600 - f64x8::splat(0.077215461) * t608 + f64x8::splat(5.0) * t593;
            let t629 = f64x8::splat(1.0) / t552;
            let t630 = t629 * t616;
            let t634 = t412 * t412;
            let t636 = f64x8::splat(1.0) / t634 / t365;
            let t637 = t332 * t636;
            let t638 = t330 * t637;
            let t639 = -f64x8::splat(0.051955731) + t593;
            let t640 = t339 * t639;
            let t641 = t596 * t616;
            let t645 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t549 - f64x8::splat(4.0) / f64x8::splat(9.0) * t595 * t596 + t599 * t612 * t617 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t623 * t8 * t627 * t630 + f64x8::splat(8.0) / f64x8::splat(81.0) * t638 * t640 * t641;
            let t649 = ((t351).select(f64x8::splat(0.0), -f64x8::splat(3.0) * f64x8::splat(0.125) * t6 * t360 * t645));
            let tzk0 = t350 + t649;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
