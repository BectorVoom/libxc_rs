//! HYB_MGGA_XC_GAS22 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_xc_gas22.c`
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
pub fn hyb_mgga_xc_gas22_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_0: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_x_1 = f64x8::splat(param_c_x_1);
    let param_c_x_2 = f64x8::splat(param_c_x_2);
    let param_c_x_0 = f64x8::splat(param_c_x_0);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_os_1 = f64x8::splat(param_c_os_1);
    let param_c_os_2 = f64x8::splat(param_c_os_2);
    let param_c_os_3 = f64x8::splat(param_c_os_3);
    let param_c_os_4 = f64x8::splat(param_c_os_4);
    let param_c_os_0 = f64x8::splat(param_c_os_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t3 = v_rho0 - v_rho1;
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = f64x8::splat(1.0) + t6;
            let t8 = (t7).simd_le(zeta_threshold);
            let t9 = ((v_rho0).simd_le(dens_threshold)) | (t8);
            let t10 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = f64x8::splat(M_CBRT4);
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = f64x8::splat(M_CBRT2);
            let t18 = t16 * t17;
            let t19 = t7 * t10 * t18;
            let t20 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t24 = ((t20).select(t22, f64x8::splat(2.0) * t17));
            let t25 = (simd::cbrt(t4));
            let t26 = t24 * t25;
            let t27 = f64x8::splat(1.0) / t7;
            let t28 = (simd::cbrt(t27));
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = (simd::cbrt(f64x8::splat(9.0)));
            let t31 = t30 * t30;
            let t32 = t13 * t13;
            let t33 = t31 * t32;
            let t34 = param_hyb_omega_0 * t10;
            let t35 = t33 * t34;
            let t36 = f64x8::splat(1.0) / t25;
            let t37 = t36 * t17;
            let t38 = ((t20).select(t21, t17));
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t28 * t39;
            let t43 = t35 * t37 * t40 / f64x8::splat(18.0);
            let t44 = (f64x8::splat(1.35)).simd_le(t43);
            let t45 = (f64x8::splat(1.35)).simd_lt(t43);
            let t46 = ((t45).select(t43, f64x8::splat(1.35)));
            let t47 = t46 * t46;
            let t50 = t47 * t47;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = t50 * t47;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t50 * t50;
            let t57 = f64x8::splat(1.0) / t56;
            let t60 = f64x8::splat(1.0) / t56 / t47;
            let t63 = f64x8::splat(1.0) / t56 / t50;
            let t66 = f64x8::splat(1.0) / t56 / t53;
            let t68 = t56 * t56;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = ((t45).select(f64x8::splat(1.35), t43));
            let t73 = ((f64x8::splat(M_PI)).sqrt());
            let t74 = f64x8::splat(1.0) / t72;
            let t76 = (simd::erf(t74 / f64x8::splat(2.0)));
            let t78 = t72 * t72;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = (simd::exp(-t79 / f64x8::splat(4.0)));
            let t82 = t81 - f64x8::splat(1.0);
            let t85 = t81 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t78 * t82;
            let t88 = f64x8::splat(2.0) * t72 * t85 + t73 * t76;
            let t92 = ((t44).select(f64x8::splat(1.0) / t47 / f64x8::splat(36.0) - t51 / f64x8::splat(960.0) + t54 / f64x8::splat(26880.0) - t57 / f64x8::splat(829440.0) + t60 / f64x8::splat(28385280.0) - t63 / f64x8::splat(1073479680.0) + t66 / f64x8::splat(44590694400.0) - t69 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t72 * t88));
            let t93 = t29 * t92;
            let t94 = param_c_x_0;
            let t95 = param_c_x_1;
            let t96 = t95 * v_sigma0;
            let t97 = v_rho0 * v_rho0;
            let t98 = (simd::cbrt(v_rho0));
            let t99 = t98 * t98;
            let t101 = f64x8::splat(1.0) / t99 / t97;
            let t102 = v_sigma0 * t101;
            let t104 = f64x8::splat(1.0) + f64x8::splat(0.003840616724010807) * t102;
            let t105 = f64x8::splat(1.0) / t104;
            let t109 = param_c_x_2;
            let t110 = f64x8::splat(M_CBRT6);
            let t111 = t110 * t110;
            let t112 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t113 = (simd::cbrt(t112));
            let t114 = t113 * t113;
            let t115 = t111 * t114;
            let t116 = f64x8::splat(3.0) / f64x8::splat(10.0) * t115;
            let t118 = f64x8::splat(1.0) / t99 / v_rho0;
            let t119 = v_tau0 * t118;
            let t120 = t116 - t119;
            let t121 = t109 * t120;
            let t122 = t116 + t119;
            let t123 = f64x8::splat(1.0) / t122;
            let t125 = t94 + f64x8::splat(0.003840616724010807) * t96 * t101 * t105 + t121 * t123;
            let t126 = t93 * t125;
            let t127 = t26 * t126;
            let t130 = ((t9).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t19 * t127));
            let t132 = f64x8::splat(1.0) - t6;
            let t133 = (t132).simd_le(zeta_threshold);
            let t134 = ((v_rho1).simd_le(dens_threshold)) | (t133);
            let t136 = t132 * t10 * t18;
            let t137 = f64x8::splat(1.0) / t132;
            let t138 = (simd::cbrt(t137));
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t138 * t39;
            let t143 = t35 * t37 * t140 / f64x8::splat(18.0);
            let t144 = (f64x8::splat(1.35)).simd_le(t143);
            let t145 = (f64x8::splat(1.35)).simd_lt(t143);
            let t146 = ((t145).select(t143, f64x8::splat(1.35)));
            let t147 = t146 * t146;
            let t150 = t147 * t147;
            let t151 = f64x8::splat(1.0) / t150;
            let t153 = t150 * t147;
            let t154 = f64x8::splat(1.0) / t153;
            let t156 = t150 * t150;
            let t157 = f64x8::splat(1.0) / t156;
            let t160 = f64x8::splat(1.0) / t156 / t147;
            let t163 = f64x8::splat(1.0) / t156 / t150;
            let t166 = f64x8::splat(1.0) / t156 / t153;
            let t168 = t156 * t156;
            let t169 = f64x8::splat(1.0) / t168;
            let t172 = ((t145).select(f64x8::splat(1.35), t143));
            let t173 = f64x8::splat(1.0) / t172;
            let t175 = (simd::erf(t173 / f64x8::splat(2.0)));
            let t177 = t172 * t172;
            let t178 = f64x8::splat(1.0) / t177;
            let t180 = (simd::exp(-t178 / f64x8::splat(4.0)));
            let t181 = t180 - f64x8::splat(1.0);
            let t184 = t180 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t177 * t181;
            let t187 = f64x8::splat(2.0) * t172 * t184 + t73 * t175;
            let t191 = ((t144).select(f64x8::splat(1.0) / t147 / f64x8::splat(36.0) - t151 / f64x8::splat(960.0) + t154 / f64x8::splat(26880.0) - t157 / f64x8::splat(829440.0) + t160 / f64x8::splat(28385280.0) - t163 / f64x8::splat(1073479680.0) + t166 / f64x8::splat(44590694400.0) - t169 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t172 * t187));
            let t192 = t139 * t191;
            let t193 = t95 * v_sigma2;
            let t194 = v_rho1 * v_rho1;
            let t195 = (simd::cbrt(v_rho1));
            let t196 = t195 * t195;
            let t198 = f64x8::splat(1.0) / t196 / t194;
            let t199 = v_sigma2 * t198;
            let t201 = f64x8::splat(1.0) + f64x8::splat(0.003840616724010807) * t199;
            let t202 = f64x8::splat(1.0) / t201;
            let t207 = f64x8::splat(1.0) / t196 / v_rho1;
            let t208 = v_tau1 * t207;
            let t209 = t116 - t208;
            let t210 = t109 * t209;
            let t211 = t116 + t208;
            let t212 = f64x8::splat(1.0) / t211;
            let t214 = t94 + f64x8::splat(0.003840616724010807) * t193 * t198 * t202 + t210 * t212;
            let t215 = t192 * t214;
            let t216 = t26 * t215;
            let t219 = ((t134).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t136 * t216));
            let t220 = ((t8).select(zeta_threshold, t7));
            let t221 = t10 * t13;
            let t222 = t221 * t15;
            let t223 = f64x8::splat(1.0) / t21;
            let t224 = (simd::cbrt(t7));
            let t226 = ((t8).select(t223, f64x8::splat(1.0) / t224));
            let t228 = t222 * t37 * t226;
            let t230 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t228;
            let t231 = ((t228).sqrt());
            let t234 = ((t228) * (t228).sqrt());
            let t236 = t10 * t10;
            let t237 = t236 * t32;
            let t238 = t237 * t14;
            let t239 = t25 * t25;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t17 * t17;
            let t242 = t240 * t241;
            let t243 = t226 * t226;
            let t245 = t238 * t242 * t243;
            let t247 = f64x8::splat(3.79785) * t231 + f64x8::splat(0.8969) * t228 + f64x8::splat(0.204775) * t234 + f64x8::splat(0.123235) * t245;
            let t250 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t247;
            let t251 = (simd::ln(t250));
            let t253 = f64x8::splat(0.0621814) * t230 * t251;
            let t255 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t22, f64x8::splat(0.0)));
            let t259 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t17 - f64x8::splat(2.0));
            let t260 = (t24 + t255 - f64x8::splat(2.0)) * t259;
            let t262 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t228;
            let t267 = f64x8::splat(7.05945) * t231 + f64x8::splat(1.549425) * t228 + f64x8::splat(0.420775) * t234 + f64x8::splat(0.1562925) * t245;
            let t270 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t267;
            let t271 = (simd::ln(t270));
            let t275 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t228;
            let t280 = f64x8::splat(5.1785) * t231 + f64x8::splat(0.905775) * t228 + f64x8::splat(0.1100325) * t234 + f64x8::splat(0.1241775) * t245;
            let t283 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t280;
            let t284 = (simd::ln(t283));
            let t285 = t275 * t284;
            let t291 = -t253 + t260 * (-f64x8::splat(0.0310907) * t262 * t271 + t253 - f64x8::splat(0.0197516734986138) * t285) + f64x8::splat(0.0197516734986138) * t260 * t285;
            let t294 = ((t9).select(f64x8::splat(0.0), t220 * t291 / f64x8::splat(2.0)));
            let t295 = param_c_ss_0;
            let t296 = t295 * v_sigma0;
            let t298 = f64x8::splat(1.0) + f64x8::splat(0.46914023462026644) * t102;
            let t299 = f64x8::splat(1.0) / t298;
            let t303 = param_c_ss_1;
            let t304 = t303 * t120;
            let t306 = param_c_ss_2;
            let t307 = t120 * t120;
            let t308 = t306 * t307;
            let t309 = t122 * t122;
            let t310 = f64x8::splat(1.0) / t309;
            let t312 = param_c_ss_3;
            let t313 = v_sigma0 * v_sigma0;
            let t314 = t313 * t313;
            let t315 = t314 * t313;
            let t316 = t312 * t315;
            let t317 = t97 * t97;
            let t318 = t317 * t317;
            let t319 = t318 * t318;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t298 * t298;
            let t322 = t321 * t321;
            let t324 = f64x8::splat(1.0) / t322 / t321;
            let t325 = t320 * t324;
            let t328 = param_c_ss_4;
            let t329 = t307 * t307;
            let t330 = t328 * t329;
            let t331 = t309 * t309;
            let t332 = f64x8::splat(1.0) / t331;
            let t333 = t330 * t332;
            let t338 = f64x8::splat(0.46914023462026644) * t296 * t101 * t299 + t304 * t123 + t308 * t310 + f64x8::splat(0.010661445329398458) * t316 * t325 + f64x8::splat(0.010661445329398458) * t333 * t315 * t320 * t324;
            let t339 = t294 * t338;
            let t340 = ((t133).select(zeta_threshold, t132));
            let t341 = (simd::cbrt(t132));
            let t343 = ((t133).select(t223, f64x8::splat(1.0) / t341));
            let t345 = t222 * t37 * t343;
            let t347 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t345;
            let t348 = ((t345).sqrt());
            let t351 = ((t345) * (t345).sqrt());
            let t353 = t343 * t343;
            let t355 = t238 * t242 * t353;
            let t357 = f64x8::splat(3.79785) * t348 + f64x8::splat(0.8969) * t345 + f64x8::splat(0.204775) * t351 + f64x8::splat(0.123235) * t355;
            let t360 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t357;
            let t361 = (simd::ln(t360));
            let t363 = f64x8::splat(0.0621814) * t347 * t361;
            let t365 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t345;
            let t370 = f64x8::splat(7.05945) * t348 + f64x8::splat(1.549425) * t345 + f64x8::splat(0.420775) * t351 + f64x8::splat(0.1562925) * t355;
            let t373 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t370;
            let t374 = (simd::ln(t373));
            let t378 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t345;
            let t383 = f64x8::splat(5.1785) * t348 + f64x8::splat(0.905775) * t345 + f64x8::splat(0.1100325) * t351 + f64x8::splat(0.1241775) * t355;
            let t386 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t383;
            let t387 = (simd::ln(t386));
            let t388 = t378 * t387;
            let t394 = -t363 + t260 * (-f64x8::splat(0.0310907) * t365 * t374 + t363 - f64x8::splat(0.0197516734986138) * t388) + f64x8::splat(0.0197516734986138) * t260 * t388;
            let t397 = ((t134).select(f64x8::splat(0.0), t340 * t394 / f64x8::splat(2.0)));
            let t398 = t295 * v_sigma2;
            let t400 = f64x8::splat(1.0) + f64x8::splat(0.46914023462026644) * t199;
            let t401 = f64x8::splat(1.0) / t400;
            let t405 = t303 * t209;
            let t407 = t209 * t209;
            let t408 = t306 * t407;
            let t409 = t211 * t211;
            let t410 = f64x8::splat(1.0) / t409;
            let t412 = v_sigma2 * v_sigma2;
            let t413 = t412 * t412;
            let t414 = t413 * t412;
            let t415 = t312 * t414;
            let t416 = t194 * t194;
            let t417 = t416 * t416;
            let t418 = t417 * t417;
            let t419 = f64x8::splat(1.0) / t418;
            let t420 = t400 * t400;
            let t421 = t420 * t420;
            let t423 = f64x8::splat(1.0) / t421 / t420;
            let t424 = t419 * t423;
            let t427 = t407 * t407;
            let t428 = t328 * t427;
            let t429 = t409 * t409;
            let t430 = f64x8::splat(1.0) / t429;
            let t431 = t428 * t430;
            let t436 = f64x8::splat(0.46914023462026644) * t398 * t198 * t401 + t405 * t212 + t408 * t410 + f64x8::splat(0.010661445329398458) * t415 * t424 + f64x8::splat(0.010661445329398458) * t431 * t414 * t419 * t423;
            let t437 = t397 * t436;
            let t439 = t221 * t15 * t36;
            let t441 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t439;
            let t442 = ((t439).sqrt());
            let t445 = ((t439) * (t439).sqrt());
            let t448 = t237 * t14 * t240;
            let t450 = f64x8::splat(3.79785) * t442 + f64x8::splat(0.8969) * t439 + f64x8::splat(0.204775) * t445 + f64x8::splat(0.123235) * t448;
            let t453 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t450;
            let t454 = (simd::ln(t453));
            let t456 = f64x8::splat(0.0621814) * t441 * t454;
            let t457 = t3 * t3;
            let t458 = t457 * t457;
            let t459 = t4 * t4;
            let t460 = t459 * t459;
            let t461 = f64x8::splat(1.0) / t460;
            let t462 = t458 * t461;
            let t463 = t224 * t7;
            let t464 = ((t8).select(t22, t463));
            let t465 = t341 * t132;
            let t466 = ((t133).select(t22, t465));
            let t467 = t464 + t466 - f64x8::splat(2.0);
            let t468 = t467 * t259;
            let t470 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t439;
            let t475 = f64x8::splat(7.05945) * t442 + f64x8::splat(1.549425) * t439 + f64x8::splat(0.420775) * t445 + f64x8::splat(0.1562925) * t448;
            let t478 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t475;
            let t479 = (simd::ln(t478));
            let t483 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t439;
            let t488 = f64x8::splat(5.1785) * t442 + f64x8::splat(0.905775) * t439 + f64x8::splat(0.1100325) * t445 + f64x8::splat(0.1241775) * t448;
            let t491 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t488;
            let t492 = (simd::ln(t491));
            let t493 = t483 * t492;
            let t495 = -f64x8::splat(0.0310907) * t470 * t479 + t456 - f64x8::splat(0.0197516734986138) * t493;
            let t496 = t468 * t495;
            let t500 = -t456 + t462 * t496 + f64x8::splat(0.0197516734986138) * t468 * t493 - t294 - t397;
            let t502 = param_c_os_1;
            let t505 = f64x8::splat(3.0) / f64x8::splat(10.0) * t115 * (t119 + t208);
            let t507 = f64x8::splat(2.0) * t119 * t208;
            let t508 = t505 - t507;
            let t509 = t508 * t508;
            let t510 = t502 * t509;
            let t511 = t505 + t507;
            let t512 = t511 * t511;
            let t513 = f64x8::splat(1.0) / t512;
            let t515 = param_c_os_2;
            let t516 = t509 * t509;
            let t517 = t516 * t509;
            let t518 = t515 * t517;
            let t519 = t512 * t512;
            let t521 = f64x8::splat(1.0) / t519 / t512;
            let t523 = param_c_os_3;
            let t524 = t523 * t517;
            let t525 = t521 * t17;
            let t526 = t102 + t199;
            let t527 = ((t526).sqrt());
            let t528 = f64x8::splat(M_SQRT2);
            let t529 = t527 * t528;
            let t530 = (simd::cbrt(t529));
            let t531 = t530 * t530;
            let t532 = t525 * t531;
            let t535 = param_c_os_4;
            let t536 = t535 * t509;
            let t537 = t513 * t17;
            let t538 = t537 * t531;
            let t541 = param_c_os_0 + t510 * t513 + t518 * t521 + t524 * t532 / f64x8::splat(2.0) + t536 * t538 / f64x8::splat(2.0);
            let t542 = t500 * t541;
            let tzk0 = t130 + t219 + t339 + t437 + t542;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
