//! MGGA_C_M05 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m05.c`
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
pub fn mgga_c_m05_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_css_1: f64,
    param_gamma_ss: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_css_0: f64,
    param_Fermi_D_cnst: f64,
    param_cab_1: f64,
    param_gamma_ab: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_cab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_css_1 = f64x8::splat(param_css_1);
    let param_gamma_ss = f64x8::splat(param_gamma_ss);
    let param_css_2 = f64x8::splat(param_css_2);
    let param_css_3 = f64x8::splat(param_css_3);
    let param_css_4 = f64x8::splat(param_css_4);
    let param_css_0 = f64x8::splat(param_css_0);
    let param_Fermi_D_cnst = f64x8::splat(param_Fermi_D_cnst);
    let param_cab_1 = f64x8::splat(param_cab_1);
    let param_gamma_ab = f64x8::splat(param_gamma_ab);
    let param_cab_2 = f64x8::splat(param_cab_2);
    let param_cab_3 = f64x8::splat(param_cab_3);
    let param_cab_4 = f64x8::splat(param_cab_4);
    let param_cab_0 = f64x8::splat(param_cab_0);
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
            let t10 = ((t8).select(zeta_threshold, t7));
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = t14 * t16;
            let t18 = (simd::cbrt(t4));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = (simd::cbrt(t7));
            let t26 = ((t8).select(t23, f64x8::splat(1.0) / t24));
            let t28 = t17 * t21 * t26;
            let t30 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t28;
            let t31 = ((t28).sqrt());
            let t34 = ((t28) * (t28).sqrt());
            let t36 = t11 * t11;
            let t37 = t13 * t13;
            let t38 = t36 * t37;
            let t39 = t38 * t15;
            let t40 = t18 * t18;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t20 * t20;
            let t43 = t41 * t42;
            let t44 = t26 * t26;
            let t46 = t39 * t43 * t44;
            let t48 = f64x8::splat(3.79785) * t31 + f64x8::splat(0.8969) * t28 + f64x8::splat(0.204775) * t34 + f64x8::splat(0.123235) * t46;
            let t51 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t48;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0621814) * t30 * t52;
            let t56 = t22 * zeta_threshold;
            let t58 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(2.0) * t20));
            let t60 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(0.0)));
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t20 - f64x8::splat(2.0));
            let t65 = (t58 + t60 - f64x8::splat(2.0)) * t64;
            let t67 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t28;
            let t72 = f64x8::splat(7.05945) * t31 + f64x8::splat(1.549425) * t28 + f64x8::splat(0.420775) * t34 + f64x8::splat(0.1562925) * t46;
            let t75 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t72;
            let t76 = (simd::ln(t75));
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t28;
            let t85 = f64x8::splat(5.1785) * t31 + f64x8::splat(0.905775) * t28 + f64x8::splat(0.1100325) * t34 + f64x8::splat(0.1241775) * t46;
            let t88 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t85;
            let t89 = (simd::ln(t88));
            let t90 = t80 * t89;
            let t96 = -t54 + t65 * (-f64x8::splat(0.0310907) * t67 * t76 + t54 - f64x8::splat(0.0197516734986138) * t90) + f64x8::splat(0.0197516734986138) * t65 * t90;
            let t99 = ((t9).select(f64x8::splat(0.0), t10 * t96 / f64x8::splat(2.0)));
            let t100 = param_css_0;
            let t101 = param_css_1;
            let t102 = t101 * param_gamma_ss;
            let t103 = v_rho0 * v_rho0;
            let t104 = (simd::cbrt(v_rho0));
            let t105 = t104 * t104;
            let t107 = f64x8::splat(1.0) / t105 / t103;
            let t108 = v_sigma0 * t107;
            let t111 = t107 * v_sigma0 * param_gamma_ss + f64x8::splat(1.0);
            let t112 = f64x8::splat(1.0) / t111;
            let t115 = param_css_2;
            let t116 = param_gamma_ss * param_gamma_ss;
            let t117 = t115 * t116;
            let t118 = v_sigma0 * v_sigma0;
            let t119 = t103 * t103;
            let t120 = t119 * v_rho0;
            let t122 = f64x8::splat(1.0) / t104 / t120;
            let t124 = t111 * t111;
            let t125 = f64x8::splat(1.0) / t124;
            let t128 = param_css_3;
            let t129 = t116 * param_gamma_ss;
            let t130 = t128 * t129;
            let t131 = t118 * v_sigma0;
            let t132 = t119 * t119;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t124 * t111;
            let t136 = f64x8::splat(1.0) / t135;
            let t139 = param_css_4;
            let t140 = t116 * t116;
            let t141 = t139 * t140;
            let t142 = t118 * t118;
            let t143 = t132 * t103;
            let t145 = f64x8::splat(1.0) / t105 / t143;
            let t147 = t124 * t124;
            let t148 = f64x8::splat(1.0) / t147;
            let t151 = t117 * t118 * t122 * t125 + t130 * t131 * t133 * t136 + t141 * t142 * t145 * t148 + t102 * t108 * t112 + t100;
            let t152 = t99 * t151;
            let t153 = f64x8::splat(1.0) / v_rho0;
            let t155 = f64x8::splat(1.0) / v_tau0;
            let t158 = f64x8::splat(1.0) - v_sigma0 * t153 * t155 / f64x8::splat(8.0);
            let t159 = v_tau0 * v_tau0;
            let t160 = t103 * v_rho0;
            let t162 = f64x8::splat(1.0) / t104 / t160;
            let t164 = param_Fermi_D_cnst * param_Fermi_D_cnst;
            let t165 = f64x8::splat(1.0) / t164;
            let t168 = (simd::exp(-f64x8::splat(4.0) * t159 * t162 * t165));
            let t169 = f64x8::splat(1.0) - t168;
            let t170 = t158 * t169;
            let t171 = t152 * t170;
            let t173 = f64x8::splat(1.0) - t6;
            let t174 = (t173).simd_le(zeta_threshold);
            let t175 = ((v_rho1).simd_le(dens_threshold)) | (t174);
            let t176 = ((t174).select(zeta_threshold, t173));
            let t177 = (simd::cbrt(t173));
            let t179 = ((t174).select(t23, f64x8::splat(1.0) / t177));
            let t181 = t17 * t21 * t179;
            let t183 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t181;
            let t184 = ((t181).sqrt());
            let t187 = ((t181) * (t181).sqrt());
            let t189 = t179 * t179;
            let t191 = t39 * t43 * t189;
            let t193 = f64x8::splat(3.79785) * t184 + f64x8::splat(0.8969) * t181 + f64x8::splat(0.204775) * t187 + f64x8::splat(0.123235) * t191;
            let t196 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t193;
            let t197 = (simd::ln(t196));
            let t199 = f64x8::splat(0.0621814) * t183 * t197;
            let t201 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t181;
            let t206 = f64x8::splat(7.05945) * t184 + f64x8::splat(1.549425) * t181 + f64x8::splat(0.420775) * t187 + f64x8::splat(0.1562925) * t191;
            let t209 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t206;
            let t210 = (simd::ln(t209));
            let t214 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t181;
            let t219 = f64x8::splat(5.1785) * t184 + f64x8::splat(0.905775) * t181 + f64x8::splat(0.1100325) * t187 + f64x8::splat(0.1241775) * t191;
            let t222 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t219;
            let t223 = (simd::ln(t222));
            let t224 = t214 * t223;
            let t230 = -t199 + t65 * (-f64x8::splat(0.0310907) * t201 * t210 + t199 - f64x8::splat(0.0197516734986138) * t224) + f64x8::splat(0.0197516734986138) * t65 * t224;
            let t233 = ((t175).select(f64x8::splat(0.0), t176 * t230 / f64x8::splat(2.0)));
            let t234 = v_rho1 * v_rho1;
            let t235 = (simd::cbrt(v_rho1));
            let t236 = t235 * t235;
            let t238 = f64x8::splat(1.0) / t236 / t234;
            let t239 = v_sigma2 * t238;
            let t242 = t238 * v_sigma2 * param_gamma_ss + f64x8::splat(1.0);
            let t243 = f64x8::splat(1.0) / t242;
            let t246 = v_sigma2 * v_sigma2;
            let t247 = t234 * t234;
            let t248 = t247 * v_rho1;
            let t250 = f64x8::splat(1.0) / t235 / t248;
            let t252 = t242 * t242;
            let t253 = f64x8::splat(1.0) / t252;
            let t256 = t246 * v_sigma2;
            let t257 = t247 * t247;
            let t258 = f64x8::splat(1.0) / t257;
            let t260 = t252 * t242;
            let t261 = f64x8::splat(1.0) / t260;
            let t264 = t246 * t246;
            let t265 = t257 * t234;
            let t267 = f64x8::splat(1.0) / t236 / t265;
            let t269 = t252 * t252;
            let t270 = f64x8::splat(1.0) / t269;
            let t273 = t117 * t246 * t250 * t253 + t130 * t256 * t258 * t261 + t141 * t264 * t267 * t270 + t102 * t239 * t243 + t100;
            let t274 = t233 * t273;
            let t275 = f64x8::splat(1.0) / v_rho1;
            let t277 = f64x8::splat(1.0) / v_tau1;
            let t280 = f64x8::splat(1.0) - v_sigma2 * t275 * t277 / f64x8::splat(8.0);
            let t281 = v_tau1 * v_tau1;
            let t282 = t234 * v_rho1;
            let t284 = f64x8::splat(1.0) / t235 / t282;
            let t288 = (simd::exp(-f64x8::splat(4.0) * t281 * t284 * t165));
            let t289 = f64x8::splat(1.0) - t288;
            let t290 = t280 * t289;
            let t291 = t274 * t290;
            let t293 = t14 * t16 * t19;
            let t295 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t293;
            let t296 = ((t293).sqrt());
            let t299 = ((t293) * (t293).sqrt());
            let t302 = t38 * t15 * t41;
            let t304 = f64x8::splat(3.79785) * t296 + f64x8::splat(0.8969) * t293 + f64x8::splat(0.204775) * t299 + f64x8::splat(0.123235) * t302;
            let t307 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t304;
            let t308 = (simd::ln(t307));
            let t310 = f64x8::splat(0.0621814) * t295 * t308;
            let t311 = t3 * t3;
            let t312 = t311 * t311;
            let t313 = t4 * t4;
            let t314 = t313 * t313;
            let t315 = f64x8::splat(1.0) / t314;
            let t316 = t312 * t315;
            let t317 = t24 * t7;
            let t318 = ((t8).select(t56, t317));
            let t319 = t177 * t173;
            let t320 = ((t174).select(t56, t319));
            let t321 = t318 + t320 - f64x8::splat(2.0);
            let t322 = t321 * t64;
            let t324 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t293;
            let t329 = f64x8::splat(7.05945) * t296 + f64x8::splat(1.549425) * t293 + f64x8::splat(0.420775) * t299 + f64x8::splat(0.1562925) * t302;
            let t332 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t329;
            let t333 = (simd::ln(t332));
            let t337 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t293;
            let t342 = f64x8::splat(5.1785) * t296 + f64x8::splat(0.905775) * t293 + f64x8::splat(0.1100325) * t299 + f64x8::splat(0.1241775) * t302;
            let t345 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t342;
            let t346 = (simd::ln(t345));
            let t347 = t337 * t346;
            let t349 = -f64x8::splat(0.0310907) * t324 * t333 + t310 - f64x8::splat(0.0197516734986138) * t347;
            let t350 = t322 * t349;
            let t354 = -t310 + t316 * t350 + f64x8::splat(0.0197516734986138) * t322 * t347 - t99 - t233;
            let t356 = param_cab_1;
            let t357 = t356 * param_gamma_ab;
            let t358 = t108 + t239;
            let t360 = param_gamma_ab * t358 + f64x8::splat(1.0);
            let t361 = f64x8::splat(1.0) / t360;
            let t364 = param_cab_2;
            let t365 = param_gamma_ab * param_gamma_ab;
            let t366 = t364 * t365;
            let t367 = t358 * t358;
            let t368 = t360 * t360;
            let t369 = f64x8::splat(1.0) / t368;
            let t372 = param_cab_3;
            let t373 = t365 * param_gamma_ab;
            let t374 = t372 * t373;
            let t375 = t367 * t358;
            let t376 = t368 * t360;
            let t377 = f64x8::splat(1.0) / t376;
            let t380 = param_cab_4;
            let t381 = t365 * t365;
            let t382 = t380 * t381;
            let t383 = t367 * t367;
            let t384 = t368 * t368;
            let t385 = f64x8::splat(1.0) / t384;
            let t388 = t357 * t358 * t361 + t366 * t367 * t369 + t374 * t375 * t377 + t382 * t383 * t385 + param_cab_0;
            let t389 = t354 * t388;
            let tzk0 = t171 + t291 + t389;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
