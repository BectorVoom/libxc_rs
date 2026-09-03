//! MGGA_C_M05 vxc pol kernel — explicit SIMD (bit-exact).
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_m05_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t390 = f64x8::splat(1.0) / t313;
            let t391 = t3 * t390;
            let t392 = t5 - t391;
            let t393 = ((t8).select(f64x8::splat(0.0), t392));
            let t396 = f64x8::splat(1.0) / t18 / t4;
            let t397 = t396 * t20;
            let t399 = t17 * t397 * t26;
            let t400 = f64x8::splat(0.017808333333333332) * t399;
            let t401 = f64x8::splat(1.0) / t317;
            let t404 = ((t8).select(f64x8::splat(0.0), -t401 * t392 / f64x8::splat(3.0)));
            let t406 = t17 * t21 * t404;
            let t408 = -t400 + f64x8::splat(0.053425) * t406;
            let t410 = f64x8::splat(0.0621814) * t408 * t52;
            let t411 = t48 * t48;
            let t412 = f64x8::splat(1.0) / t411;
            let t413 = t30 * t412;
            let t414 = f64x8::splat(1.0) / t31;
            let t415 = t399 / f64x8::splat(3.0);
            let t416 = -t415 + t406;
            let t417 = t414 * t416;
            let t419 = f64x8::splat(0.29896666666666666) * t399;
            let t421 = ((t28).sqrt());
            let t422 = t421 * t416;
            let t425 = f64x8::splat(1.0) / t40 / t4;
            let t426 = t425 * t42;
            let t428 = t39 * t426 * t44;
            let t429 = f64x8::splat(0.08215666666666667) * t428;
            let t430 = t26 * t404;
            let t432 = t39 * t43 * t430;
            let t434 = f64x8::splat(1.898925) * t417 - t419 + f64x8::splat(0.8969) * t406 + f64x8::splat(0.3071625) * t422 - t429 + f64x8::splat(0.24647) * t432;
            let t435 = f64x8::splat(1.0) / t51;
            let t436 = t434 * t435;
            let t438 = f64x8::splat(1.0) * t413 * t436;
            let t439 = f64x8::splat(0.017123333333333334) * t399;
            let t441 = -t439 + f64x8::splat(0.05137) * t406;
            let t444 = t72 * t72;
            let t445 = f64x8::splat(1.0) / t444;
            let t446 = t67 * t445;
            let t448 = f64x8::splat(0.516475) * t399;
            let t451 = f64x8::splat(0.104195) * t428;
            let t453 = f64x8::splat(3.529725) * t417 - t448 + f64x8::splat(1.549425) * t406 + f64x8::splat(0.6311625) * t422 - t451 + f64x8::splat(0.312585) * t432;
            let t454 = f64x8::splat(1.0) / t75;
            let t455 = t453 * t454;
            let t458 = f64x8::splat(0.009270833333333334) * t399;
            let t460 = -t458 + f64x8::splat(0.0278125) * t406;
            let t461 = t460 * t89;
            let t463 = t85 * t85;
            let t464 = f64x8::splat(1.0) / t463;
            let t465 = t80 * t464;
            let t467 = f64x8::splat(0.301925) * t399;
            let t470 = f64x8::splat(0.082785) * t428;
            let t472 = f64x8::splat(2.58925) * t417 - t467 + f64x8::splat(0.905775) * t406 + f64x8::splat(0.16504875) * t422 - t470 + f64x8::splat(0.248355) * t432;
            let t473 = f64x8::splat(1.0) / t88;
            let t474 = t472 * t473;
            let t481 = t65 * t80;
            let t483 = t464 * t472 * t473;
            let t486 = -t410 + t438 + t65 * (-f64x8::splat(0.0310907) * t441 * t76 + f64x8::splat(1.0) * t446 * t455 + t410 - t438 - f64x8::splat(0.0197516734986138) * t461 + f64x8::splat(0.5848223622634646) * t465 * t474) + f64x8::splat(0.0197516734986138) * t65 * t461 - f64x8::splat(0.5848223622634646) * t481 * t483;
            let t490 = ((t9).select(f64x8::splat(0.0), t10 * t486 / f64x8::splat(2.0) + t393 * t96 / f64x8::splat(2.0)));
            let t491 = t490 * t151;
            let t492 = t491 * t170;
            let t494 = f64x8::splat(1.0) / t105 / t160;
            let t495 = v_sigma0 * t494;
            let t499 = t101 * t116;
            let t500 = t119 * t103;
            let t502 = f64x8::splat(1.0) / t104 / t500;
            let t504 = t118 * t502 * t125;
            let t509 = t115 * t129;
            let t510 = t132 * v_rho0;
            let t511 = f64x8::splat(1.0) / t510;
            let t513 = t131 * t511 * t136;
            let t518 = t128 * t140;
            let t519 = t132 * t160;
            let t521 = f64x8::splat(1.0) / t105 / t519;
            let t523 = t142 * t521 * t148;
            let t528 = t140 * param_gamma_ss;
            let t529 = t139 * t528;
            let t530 = t142 * v_sigma0;
            let t531 = t132 * t500;
            let t533 = f64x8::splat(1.0) / t104 / t531;
            let t536 = f64x8::splat(1.0) / t147 / t111;
            let t540 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t102 * t495 * t112 + f64x8::splat(8.0) / f64x8::splat(3.0) * t499 * t504 - f64x8::splat(16.0) / f64x8::splat(3.0) * t117 * t504 + f64x8::splat(16.0) / f64x8::splat(3.0) * t509 * t513 - f64x8::splat(8.0) * t130 * t513 + f64x8::splat(8.0) * t518 * t523 - f64x8::splat(32.0) / f64x8::splat(3.0) * t141 * t523 + f64x8::splat(32.0) / f64x8::splat(3.0) * t529 * t530 * t533 * t536;
            let t541 = t99 * t540;
            let t542 = t541 * t170;
            let t543 = t152 * v_sigma0;
            let t544 = f64x8::splat(1.0) / t103;
            let t546 = t544 * t155 * t169;
            let t547 = t543 * t546;
            let t548 = t547 / f64x8::splat(8.0);
            let t549 = t152 * t158;
            let t551 = f64x8::splat(1.0) / t104 / t119;
            let t553 = t165 * t168;
            let t554 = t159 * t551 * t553;
            let t555 = t549 * t554;
            let t556 = f64x8::splat(40.0) / f64x8::splat(3.0) * t555;
            let t557 = -t392;
            let t558 = ((t174).select(f64x8::splat(0.0), t557));
            let t561 = t17 * t397 * t179;
            let t562 = f64x8::splat(0.017808333333333332) * t561;
            let t563 = f64x8::splat(1.0) / t319;
            let t566 = ((t174).select(f64x8::splat(0.0), -t563 * t557 / f64x8::splat(3.0)));
            let t568 = t17 * t21 * t566;
            let t570 = -t562 + f64x8::splat(0.053425) * t568;
            let t572 = f64x8::splat(0.0621814) * t570 * t197;
            let t573 = t193 * t193;
            let t574 = f64x8::splat(1.0) / t573;
            let t575 = t183 * t574;
            let t576 = f64x8::splat(1.0) / t184;
            let t577 = t561 / f64x8::splat(3.0);
            let t578 = -t577 + t568;
            let t579 = t576 * t578;
            let t581 = f64x8::splat(0.29896666666666666) * t561;
            let t583 = ((t181).sqrt());
            let t584 = t583 * t578;
            let t587 = t39 * t426 * t189;
            let t588 = f64x8::splat(0.08215666666666667) * t587;
            let t589 = t179 * t566;
            let t591 = t39 * t43 * t589;
            let t593 = f64x8::splat(1.898925) * t579 - t581 + f64x8::splat(0.8969) * t568 + f64x8::splat(0.3071625) * t584 - t588 + f64x8::splat(0.24647) * t591;
            let t594 = f64x8::splat(1.0) / t196;
            let t595 = t593 * t594;
            let t597 = f64x8::splat(1.0) * t575 * t595;
            let t598 = f64x8::splat(0.017123333333333334) * t561;
            let t600 = -t598 + f64x8::splat(0.05137) * t568;
            let t603 = t206 * t206;
            let t604 = f64x8::splat(1.0) / t603;
            let t605 = t201 * t604;
            let t607 = f64x8::splat(0.516475) * t561;
            let t610 = f64x8::splat(0.104195) * t587;
            let t612 = f64x8::splat(3.529725) * t579 - t607 + f64x8::splat(1.549425) * t568 + f64x8::splat(0.6311625) * t584 - t610 + f64x8::splat(0.312585) * t591;
            let t613 = f64x8::splat(1.0) / t209;
            let t614 = t612 * t613;
            let t617 = f64x8::splat(0.009270833333333334) * t561;
            let t619 = -t617 + f64x8::splat(0.0278125) * t568;
            let t620 = t619 * t223;
            let t622 = t219 * t219;
            let t623 = f64x8::splat(1.0) / t622;
            let t624 = t214 * t623;
            let t626 = f64x8::splat(0.301925) * t561;
            let t629 = f64x8::splat(0.082785) * t587;
            let t631 = f64x8::splat(2.58925) * t579 - t626 + f64x8::splat(0.905775) * t568 + f64x8::splat(0.16504875) * t584 - t629 + f64x8::splat(0.248355) * t591;
            let t632 = f64x8::splat(1.0) / t222;
            let t633 = t631 * t632;
            let t640 = t65 * t214;
            let t642 = t623 * t631 * t632;
            let t645 = -t572 + t597 + t65 * (-f64x8::splat(0.0310907) * t600 * t210 + f64x8::splat(1.0) * t605 * t614 + t572 - t597 - f64x8::splat(0.0197516734986138) * t620 + f64x8::splat(0.5848223622634646) * t624 * t633) + f64x8::splat(0.0197516734986138) * t65 * t620 - f64x8::splat(0.5848223622634646) * t640 * t642;
            let t649 = ((t175).select(f64x8::splat(0.0), t176 * t645 / f64x8::splat(2.0) + t558 * t230 / f64x8::splat(2.0)));
            let t650 = t649 * t273;
            let t651 = t650 * t290;
            let t652 = t16 * t396;
            let t655 = f64x8::splat(0.0011073470983333333) * t14 * t652 * t308;
            let t656 = t304 * t304;
            let t657 = f64x8::splat(1.0) / t656;
            let t658 = t295 * t657;
            let t660 = f64x8::splat(1.0) / t296 * t11;
            let t661 = t13 * t16;
            let t662 = t661 * t396;
            let t663 = t660 * t662;
            let t665 = t14 * t652;
            let t667 = ((t293).sqrt());
            let t668 = t667 * t11;
            let t669 = t668 * t662;
            let t672 = t38 * t15 * t425;
            let t674 = -f64x8::splat(0.632975) * t663 - f64x8::splat(0.29896666666666666) * t665 - f64x8::splat(0.1023875) * t669 - f64x8::splat(0.08215666666666667) * t672;
            let t675 = f64x8::splat(1.0) / t307;
            let t676 = t674 * t675;
            let t678 = f64x8::splat(1.0) * t658 * t676;
            let t679 = t311 * t3;
            let t680 = t679 * t315;
            let t682 = f64x8::splat(4.0) * t680 * t350;
            let t683 = t314 * t4;
            let t684 = f64x8::splat(1.0) / t683;
            let t685 = t312 * t684;
            let t687 = f64x8::splat(4.0) * t685 * t350;
            let t690 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t392));
            let t693 = ((t174).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t177 * t557));
            let t695 = (t690 + t693) * t64;
            let t696 = t695 * t349;
            let t701 = t329 * t329;
            let t702 = f64x8::splat(1.0) / t701;
            let t703 = t324 * t702;
            let t708 = -f64x8::splat(1.176575) * t663 - f64x8::splat(0.516475) * t665 - f64x8::splat(0.2103875) * t669 - f64x8::splat(0.104195) * t672;
            let t709 = f64x8::splat(1.0) / t332;
            let t710 = t708 * t709;
            let t716 = t342 * t342;
            let t717 = f64x8::splat(1.0) / t716;
            let t718 = t337 * t717;
            let t723 = -f64x8::splat(0.8630833333333333) * t663 - f64x8::splat(0.301925) * t665 - f64x8::splat(0.05501625) * t669 - f64x8::splat(0.082785) * t672;
            let t724 = f64x8::splat(1.0) / t345;
            let t725 = t723 * t724;
            let t728 = f64x8::splat(0.0005323764196666666) * t14 * t652 * t333 + f64x8::splat(1.0) * t703 * t710 - t655 - t678 + f64x8::splat(0.00018311447306006544) * t14 * t652 * t346 + f64x8::splat(0.5848223622634646) * t718 * t725;
            let t729 = t322 * t728;
            let t730 = t316 * t729;
            let t733 = t322 * t11;
            let t735 = t661 * t396 * t346;
            let t737 = f64x8::splat(0.00018311447306006544) * t733 * t735;
            let t738 = t322 * t337;
            let t740 = t717 * t723 * t724;
            let t742 = f64x8::splat(0.5848223622634646) * t738 * t740;
            let t743 = t655 + t678 + t682 - t687 + t316 * t696 + t730 + f64x8::splat(0.0197516734986138) * t695 * t347 - t737 - t742 - t490 - t649;
            let t744 = t743 * t388;
            let t748 = t356 * t365;
            let t749 = t748 * t358;
            let t750 = t369 * v_sigma0;
            let t751 = t750 * t494;
            let t754 = t366 * t358;
            let t757 = t364 * t373;
            let t758 = t757 * t367;
            let t759 = t377 * v_sigma0;
            let t760 = t759 * t494;
            let t763 = t374 * t367;
            let t766 = t372 * t381;
            let t767 = t766 * t375;
            let t768 = t385 * v_sigma0;
            let t769 = t768 * t494;
            let t772 = t382 * t375;
            let t775 = t381 * param_gamma_ab;
            let t776 = t380 * t775;
            let t777 = t776 * t383;
            let t779 = f64x8::splat(1.0) / t384 / t360;
            let t780 = t779 * v_sigma0;
            let t784 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t357 * t495 * t361 + f64x8::splat(8.0) / f64x8::splat(3.0) * t749 * t751 - f64x8::splat(16.0) / f64x8::splat(3.0) * t754 * t751 + f64x8::splat(16.0) / f64x8::splat(3.0) * t758 * t760 - f64x8::splat(8.0) * t763 * t760 + f64x8::splat(8.0) * t767 * t769 - f64x8::splat(32.0) / f64x8::splat(3.0) * t772 * t769 + f64x8::splat(32.0) / f64x8::splat(3.0) * t777 * t780 * t494;
            let t785 = t354 * t784;
            let tvrho0 = t171 + t291 + t389 + t4 * (t492 + t542 + t548 - t556 + t651 + t744 + t785);
            acc_vrho_0 = tvrho0;
            let t788 = -t5 - t391;
            let t789 = ((t8).select(f64x8::splat(0.0), t788));
            let t793 = ((t8).select(f64x8::splat(0.0), -t401 * t788 / f64x8::splat(3.0)));
            let t795 = t17 * t21 * t793;
            let t797 = -t400 + f64x8::splat(0.053425) * t795;
            let t799 = f64x8::splat(0.0621814) * t797 * t52;
            let t800 = -t415 + t795;
            let t801 = t414 * t800;
            let t804 = t421 * t800;
            let t806 = t26 * t793;
            let t808 = t39 * t43 * t806;
            let t810 = f64x8::splat(1.898925) * t801 - t419 + f64x8::splat(0.8969) * t795 + f64x8::splat(0.3071625) * t804 - t429 + f64x8::splat(0.24647) * t808;
            let t811 = t810 * t435;
            let t813 = f64x8::splat(1.0) * t413 * t811;
            let t815 = -t439 + f64x8::splat(0.05137) * t795;
            let t822 = f64x8::splat(3.529725) * t801 - t448 + f64x8::splat(1.549425) * t795 + f64x8::splat(0.6311625) * t804 - t451 + f64x8::splat(0.312585) * t808;
            let t823 = t822 * t454;
            let t827 = -t458 + f64x8::splat(0.0278125) * t795;
            let t828 = t827 * t89;
            let t834 = f64x8::splat(2.58925) * t801 - t467 + f64x8::splat(0.905775) * t795 + f64x8::splat(0.16504875) * t804 - t470 + f64x8::splat(0.248355) * t808;
            let t835 = t834 * t473;
            let t843 = t464 * t834 * t473;
            let t846 = -t799 + t813 + t65 * (-f64x8::splat(0.0310907) * t815 * t76 + f64x8::splat(1.0) * t446 * t823 + t799 - t813 - f64x8::splat(0.0197516734986138) * t828 + f64x8::splat(0.5848223622634646) * t465 * t835) + f64x8::splat(0.0197516734986138) * t65 * t828 - f64x8::splat(0.5848223622634646) * t481 * t843;
            let t850 = ((t9).select(f64x8::splat(0.0), t10 * t846 / f64x8::splat(2.0) + t789 * t96 / f64x8::splat(2.0)));
            let t851 = t850 * t151;
            let t852 = t851 * t170;
            let t853 = -t788;
            let t854 = ((t174).select(f64x8::splat(0.0), t853));
            let t858 = ((t174).select(f64x8::splat(0.0), -t563 * t853 / f64x8::splat(3.0)));
            let t860 = t17 * t21 * t858;
            let t862 = -t562 + f64x8::splat(0.053425) * t860;
            let t864 = f64x8::splat(0.0621814) * t862 * t197;
            let t865 = -t577 + t860;
            let t866 = t576 * t865;
            let t869 = t583 * t865;
            let t871 = t179 * t858;
            let t873 = t39 * t43 * t871;
            let t875 = f64x8::splat(1.898925) * t866 - t581 + f64x8::splat(0.8969) * t860 + f64x8::splat(0.3071625) * t869 - t588 + f64x8::splat(0.24647) * t873;
            let t876 = t875 * t594;
            let t878 = f64x8::splat(1.0) * t575 * t876;
            let t880 = -t598 + f64x8::splat(0.05137) * t860;
            let t887 = f64x8::splat(3.529725) * t866 - t607 + f64x8::splat(1.549425) * t860 + f64x8::splat(0.6311625) * t869 - t610 + f64x8::splat(0.312585) * t873;
            let t888 = t887 * t613;
            let t892 = -t617 + f64x8::splat(0.0278125) * t860;
            let t893 = t892 * t223;
            let t899 = f64x8::splat(2.58925) * t866 - t626 + f64x8::splat(0.905775) * t860 + f64x8::splat(0.16504875) * t869 - t629 + f64x8::splat(0.248355) * t873;
            let t900 = t899 * t632;
            let t908 = t623 * t899 * t632;
            let t911 = -t864 + t878 + t65 * (-f64x8::splat(0.0310907) * t880 * t210 + f64x8::splat(1.0) * t605 * t888 + t864 - t878 - f64x8::splat(0.0197516734986138) * t893 + f64x8::splat(0.5848223622634646) * t624 * t900) + f64x8::splat(0.0197516734986138) * t65 * t893 - f64x8::splat(0.5848223622634646) * t640 * t908;
            let t915 = ((t175).select(f64x8::splat(0.0), t176 * t911 / f64x8::splat(2.0) + t854 * t230 / f64x8::splat(2.0)));
            let t916 = t915 * t273;
            let t917 = t916 * t290;
            let t919 = f64x8::splat(1.0) / t236 / t282;
            let t920 = v_sigma2 * t919;
            let t924 = t247 * t234;
            let t926 = f64x8::splat(1.0) / t235 / t924;
            let t928 = t246 * t926 * t253;
            let t933 = t257 * v_rho1;
            let t934 = f64x8::splat(1.0) / t933;
            let t936 = t256 * t934 * t261;
            let t941 = t257 * t282;
            let t943 = f64x8::splat(1.0) / t236 / t941;
            let t945 = t264 * t943 * t270;
            let t950 = t264 * v_sigma2;
            let t951 = t257 * t924;
            let t953 = f64x8::splat(1.0) / t235 / t951;
            let t956 = f64x8::splat(1.0) / t269 / t242;
            let t960 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t102 * t920 * t243 + f64x8::splat(8.0) / f64x8::splat(3.0) * t499 * t928 - f64x8::splat(16.0) / f64x8::splat(3.0) * t117 * t928 + f64x8::splat(16.0) / f64x8::splat(3.0) * t509 * t936 - f64x8::splat(8.0) * t130 * t936 + f64x8::splat(8.0) * t518 * t945 - f64x8::splat(32.0) / f64x8::splat(3.0) * t141 * t945 + f64x8::splat(32.0) / f64x8::splat(3.0) * t529 * t950 * t953 * t956;
            let t961 = t233 * t960;
            let t962 = t961 * t290;
            let t963 = t274 * v_sigma2;
            let t964 = f64x8::splat(1.0) / t234;
            let t966 = t964 * t277 * t289;
            let t967 = t963 * t966;
            let t968 = t967 / f64x8::splat(8.0);
            let t969 = t274 * t280;
            let t971 = f64x8::splat(1.0) / t235 / t247;
            let t973 = t165 * t288;
            let t974 = t281 * t971 * t973;
            let t975 = t969 * t974;
            let t976 = f64x8::splat(40.0) / f64x8::splat(3.0) * t975;
            let t979 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t788));
            let t982 = ((t174).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t177 * t853));
            let t984 = (t979 + t982) * t64;
            let t985 = t984 * t349;
            let t989 = t655 + t678 - t682 - t687 + t316 * t985 + t730 + f64x8::splat(0.0197516734986138) * t984 * t347 - t737 - t742 - t850 - t915;
            let t990 = t989 * t388;
            let t994 = t369 * v_sigma2;
            let t995 = t994 * t919;
            let t1000 = t377 * v_sigma2;
            let t1001 = t1000 * t919;
            let t1006 = t385 * v_sigma2;
            let t1007 = t1006 * t919;
            let t1012 = t779 * v_sigma2;
            let t1016 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t357 * t920 * t361 + f64x8::splat(8.0) / f64x8::splat(3.0) * t749 * t995 - f64x8::splat(16.0) / f64x8::splat(3.0) * t754 * t995 + f64x8::splat(16.0) / f64x8::splat(3.0) * t758 * t1001 - f64x8::splat(8.0) * t763 * t1001 + f64x8::splat(8.0) * t767 * t1007 - f64x8::splat(32.0) / f64x8::splat(3.0) * t772 * t1007 + f64x8::splat(32.0) / f64x8::splat(3.0) * t777 * t1012 * t919;
            let t1017 = t354 * t1016;
            let tvrho1 = t171 + t291 + t389 + t4 * (t852 + t917 + t962 + t968 - t976 + t990 + t1017);
            acc_vrho_1 = tvrho1;
            let t1023 = v_sigma0 * t122 * t125;
            let t1028 = t118 * t133 * t136;
            let t1034 = t131 * t145 * t148;
            let t1039 = t132 * t120;
            let t1041 = f64x8::splat(1.0) / t104 / t1039;
            let t1046 = -f64x8::splat(4.0) * t1041 * t142 * t529 * t536 + t102 * t107 * t112 + f64x8::splat(2.0) * t117 * t1023 - t499 * t1023 + f64x8::splat(3.0) * t130 * t1028 - f64x8::splat(2.0) * t509 * t1028 + f64x8::splat(4.0) * t141 * t1034 - f64x8::splat(3.0) * t518 * t1034;
            let t1047 = t99 * t1046;
            let t1048 = t1047 * t170;
            let t1050 = t153 * t155 * t169;
            let t1052 = t152 * t1050 / f64x8::splat(8.0);
            let t1055 = t358 * t369;
            let t1056 = t1055 * t107;
            let t1060 = t367 * t377;
            let t1061 = t1060 * t107;
            let t1066 = t375 * t385;
            let t1067 = t1066 * t107;
            let t1072 = t383 * t779;
            let t1076 = -f64x8::splat(4.0) * t776 * t1072 * t107 + t357 * t107 * t361 + f64x8::splat(2.0) * t366 * t1056 - t748 * t1056 + f64x8::splat(3.0) * t374 * t1061 - f64x8::splat(2.0) * t757 * t1061 + f64x8::splat(4.0) * t382 * t1067 - f64x8::splat(3.0) * t766 * t1067;
            let t1077 = t354 * t1076;
            let tvsigma0 = t4 * (t1048 - t1052 + t1077);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1082 = v_sigma2 * t250 * t253;
            let t1087 = t246 * t258 * t261;
            let t1093 = t256 * t267 * t270;
            let t1098 = t257 * t248;
            let t1100 = f64x8::splat(1.0) / t235 / t1098;
            let t1105 = -f64x8::splat(4.0) * t529 * t264 * t1100 * t956 + t102 * t238 * t243 + f64x8::splat(2.0) * t117 * t1082 - t499 * t1082 + f64x8::splat(3.0) * t130 * t1087 - f64x8::splat(2.0) * t509 * t1087 + f64x8::splat(4.0) * t141 * t1093 - f64x8::splat(3.0) * t518 * t1093;
            let t1106 = t233 * t1105;
            let t1107 = t1106 * t290;
            let t1109 = t275 * t277 * t289;
            let t1111 = t274 * t1109 / f64x8::splat(8.0);
            let t1114 = t1055 * t238;
            let t1118 = t1060 * t238;
            let t1123 = t1066 * t238;
            let t1131 = -f64x8::splat(4.0) * t776 * t1072 * t238 + t357 * t238 * t361 + f64x8::splat(2.0) * t366 * t1114 - t748 * t1114 + f64x8::splat(3.0) * t374 * t1118 - f64x8::splat(2.0) * t757 * t1118 + f64x8::splat(4.0) * t382 * t1123 - f64x8::splat(3.0) * t766 * t1123;
            let t1132 = t354 * t1131;
            let tvsigma2 = t4 * (t1107 - t1111 + t1132);
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t1134 = f64x8::splat(1.0) / t159;
            let t1136 = t153 * t1134 * t169;
            let t1138 = t543 * t1136 / f64x8::splat(8.0);
            let t1140 = v_tau0 * t162 * t553;
            let t1142 = f64x8::splat(8.0) * t549 * t1140;
            let tvtau0 = t4 * (t1138 + t1142);
            acc_vtau_0 = tvtau0;
            let t1144 = f64x8::splat(1.0) / t281;
            let t1146 = t275 * t1144 * t289;
            let t1148 = t963 * t1146 / f64x8::splat(8.0);
            let t1150 = v_tau1 * t284 * t973;
            let t1152 = f64x8::splat(8.0) * t969 * t1150;
            let tvtau1 = t4 * (t1148 + t1152);
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
