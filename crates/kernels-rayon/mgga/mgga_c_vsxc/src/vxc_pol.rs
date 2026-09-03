//! MGGA_C_VSXC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`
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
pub fn mgga_c_vsxc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_dss_0: f64,
    param_alpha_ss: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_dab_0: f64,
    param_alpha_ab: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_dss_0 = f64x8::splat(param_dss_0);
    let param_alpha_ss = f64x8::splat(param_alpha_ss);
    let param_dss_1 = f64x8::splat(param_dss_1);
    let param_dss_2 = f64x8::splat(param_dss_2);
    let param_dss_3 = f64x8::splat(param_dss_3);
    let param_dss_4 = f64x8::splat(param_dss_4);
    let param_dss_5 = f64x8::splat(param_dss_5);
    let param_dab_0 = f64x8::splat(param_dab_0);
    let param_alpha_ab = f64x8::splat(param_alpha_ab);
    let param_dab_1 = f64x8::splat(param_dab_1);
    let param_dab_2 = f64x8::splat(param_dab_2);
    let param_dab_3 = f64x8::splat(param_dab_3);
    let param_dab_4 = f64x8::splat(param_dab_4);
    let param_dab_5 = f64x8::splat(param_dab_5);
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
            let t100 = param_dss_0;
            let t101 = v_rho0 * v_rho0;
            let t102 = (simd::cbrt(v_rho0));
            let t103 = t102 * t102;
            let t105 = f64x8::splat(1.0) / t103 / t101;
            let t106 = v_sigma0 * t105;
            let t108 = f64x8::splat(1.0) / t103 / v_rho0;
            let t110 = f64x8::splat(2.0) * v_tau0 * t108;
            let t111 = f64x8::splat(M_CBRT6);
            let t112 = t111 * t111;
            let t113 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t114 = (simd::cbrt(t113));
            let t115 = t114 * t114;
            let t116 = t112 * t115;
            let t117 = f64x8::splat(3.0) / f64x8::splat(5.0) * t116;
            let t120 = f64x8::splat(1.0) + param_alpha_ss * (t106 + t110 - t117);
            let t123 = param_dss_1;
            let t124 = t123 * v_sigma0;
            let t126 = param_dss_2;
            let t127 = t110 - t117;
            let t129 = t124 * t105 + t126 * t127;
            let t130 = t120 * t120;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = param_dss_3;
            let t134 = v_sigma0 * v_sigma0;
            let t135 = t133 * t134;
            let t136 = t101 * t101;
            let t137 = t136 * v_rho0;
            let t139 = f64x8::splat(1.0) / t102 / t137;
            let t141 = param_dss_4;
            let t142 = t141 * v_sigma0;
            let t145 = param_dss_5;
            let t146 = t127 * t127;
            let t148 = t142 * t105 * t127 + t135 * t139 + t145 * t146;
            let t149 = t130 * t120;
            let t150 = f64x8::splat(1.0) / t149;
            let t152 = t100 / t120 + t129 * t131 + t148 * t150;
            let t153 = t99 * t152;
            let t154 = f64x8::splat(1.0) / v_rho0;
            let t155 = v_sigma0 * t154;
            let t156 = f64x8::splat(1.0) / v_tau0;
            let t159 = f64x8::splat(1.0) - t155 * t156 / f64x8::splat(8.0);
            let t160 = t153 * t159;
            let t162 = f64x8::splat(1.0) - t6;
            let t163 = (t162).simd_le(zeta_threshold);
            let t164 = ((v_rho1).simd_le(dens_threshold)) | (t163);
            let t165 = ((t163).select(zeta_threshold, t162));
            let t166 = (simd::cbrt(t162));
            let t168 = ((t163).select(t23, f64x8::splat(1.0) / t166));
            let t170 = t17 * t21 * t168;
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t170;
            let t173 = ((t170).sqrt());
            let t176 = ((t170) * (t170).sqrt());
            let t178 = t168 * t168;
            let t180 = t39 * t43 * t178;
            let t182 = f64x8::splat(3.79785) * t173 + f64x8::splat(0.8969) * t170 + f64x8::splat(0.204775) * t176 + f64x8::splat(0.123235) * t180;
            let t185 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t182;
            let t186 = (simd::ln(t185));
            let t188 = f64x8::splat(0.0621814) * t172 * t186;
            let t190 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t170;
            let t195 = f64x8::splat(7.05945) * t173 + f64x8::splat(1.549425) * t170 + f64x8::splat(0.420775) * t176 + f64x8::splat(0.1562925) * t180;
            let t198 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t195;
            let t199 = (simd::ln(t198));
            let t203 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t170;
            let t208 = f64x8::splat(5.1785) * t173 + f64x8::splat(0.905775) * t170 + f64x8::splat(0.1100325) * t176 + f64x8::splat(0.1241775) * t180;
            let t211 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t208;
            let t212 = (simd::ln(t211));
            let t213 = t203 * t212;
            let t219 = -t188 + t65 * (-f64x8::splat(0.0310907) * t190 * t199 + t188 - f64x8::splat(0.0197516734986138) * t213) + f64x8::splat(0.0197516734986138) * t65 * t213;
            let t222 = ((t164).select(f64x8::splat(0.0), t165 * t219 / f64x8::splat(2.0)));
            let t223 = v_rho1 * v_rho1;
            let t224 = (simd::cbrt(v_rho1));
            let t225 = t224 * t224;
            let t227 = f64x8::splat(1.0) / t225 / t223;
            let t228 = v_sigma2 * t227;
            let t230 = f64x8::splat(1.0) / t225 / v_rho1;
            let t232 = f64x8::splat(2.0) * v_tau1 * t230;
            let t235 = f64x8::splat(1.0) + param_alpha_ss * (t228 + t232 - t117);
            let t238 = t123 * v_sigma2;
            let t240 = t232 - t117;
            let t242 = t126 * t240 + t238 * t227;
            let t243 = t235 * t235;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = v_sigma2 * v_sigma2;
            let t247 = t133 * t246;
            let t248 = t223 * t223;
            let t249 = t248 * v_rho1;
            let t251 = f64x8::splat(1.0) / t224 / t249;
            let t253 = t141 * v_sigma2;
            let t256 = t240 * t240;
            let t258 = t253 * t227 * t240 + t145 * t256 + t247 * t251;
            let t259 = t243 * t235;
            let t260 = f64x8::splat(1.0) / t259;
            let t262 = t100 / t235 + t242 * t244 + t258 * t260;
            let t263 = t222 * t262;
            let t264 = f64x8::splat(1.0) / v_rho1;
            let t265 = v_sigma2 * t264;
            let t266 = f64x8::splat(1.0) / v_tau1;
            let t269 = f64x8::splat(1.0) - t265 * t266 / f64x8::splat(8.0);
            let t270 = t263 * t269;
            let t272 = t14 * t16 * t19;
            let t274 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t272;
            let t275 = ((t272).sqrt());
            let t278 = ((t272) * (t272).sqrt());
            let t281 = t38 * t15 * t41;
            let t283 = f64x8::splat(3.79785) * t275 + f64x8::splat(0.8969) * t272 + f64x8::splat(0.204775) * t278 + f64x8::splat(0.123235) * t281;
            let t286 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t283;
            let t287 = (simd::ln(t286));
            let t289 = f64x8::splat(0.0621814) * t274 * t287;
            let t290 = t3 * t3;
            let t291 = t290 * t290;
            let t292 = t4 * t4;
            let t293 = t292 * t292;
            let t294 = f64x8::splat(1.0) / t293;
            let t295 = t291 * t294;
            let t296 = t24 * t7;
            let t297 = ((t8).select(t56, t296));
            let t298 = t166 * t162;
            let t299 = ((t163).select(t56, t298));
            let t300 = t297 + t299 - f64x8::splat(2.0);
            let t301 = t300 * t64;
            let t303 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t272;
            let t308 = f64x8::splat(7.05945) * t275 + f64x8::splat(1.549425) * t272 + f64x8::splat(0.420775) * t278 + f64x8::splat(0.1562925) * t281;
            let t311 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t308;
            let t312 = (simd::ln(t311));
            let t316 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t272;
            let t321 = f64x8::splat(5.1785) * t275 + f64x8::splat(0.905775) * t272 + f64x8::splat(0.1100325) * t278 + f64x8::splat(0.1241775) * t281;
            let t324 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t321;
            let t325 = (simd::ln(t324));
            let t326 = t316 * t325;
            let t328 = -f64x8::splat(0.0310907) * t303 * t312 + t289 - f64x8::splat(0.0197516734986138) * t326;
            let t329 = t301 * t328;
            let t333 = -t289 + t295 * t329 + f64x8::splat(0.0197516734986138) * t301 * t326 - t99 - t222;
            let t334 = param_dab_0;
            let t335 = f64x8::splat(6.0) / f64x8::splat(5.0) * t116;
            let t338 = f64x8::splat(1.0) + param_alpha_ab * (t106 + t228 + t110 + t232 - t335);
            let t341 = param_dab_1;
            let t342 = t106 + t228;
            let t344 = param_dab_2;
            let t345 = t110 + t232 - t335;
            let t347 = t341 * t342 + t344 * t345;
            let t348 = t338 * t338;
            let t349 = f64x8::splat(1.0) / t348;
            let t351 = param_dab_3;
            let t352 = t342 * t342;
            let t354 = param_dab_4;
            let t355 = t354 * t342;
            let t357 = param_dab_5;
            let t358 = t345 * t345;
            let t360 = t355 * t345 + t351 * t352 + t357 * t358;
            let t361 = t348 * t338;
            let t362 = f64x8::splat(1.0) / t361;
            let t364 = t334 / t338 + t347 * t349 + t360 * t362;
            let t365 = t333 * t364;
            let tzk0 = t160 + t270 + t365;
            acc_zk = tzk0;
            let t366 = f64x8::splat(1.0) / t292;
            let t367 = t3 * t366;
            let t368 = t5 - t367;
            let t369 = ((t8).select(f64x8::splat(0.0), t368));
            let t372 = f64x8::splat(1.0) / t18 / t4;
            let t373 = t372 * t20;
            let t375 = t17 * t373 * t26;
            let t376 = f64x8::splat(0.017808333333333332) * t375;
            let t377 = f64x8::splat(1.0) / t296;
            let t380 = ((t8).select(f64x8::splat(0.0), -t377 * t368 / f64x8::splat(3.0)));
            let t382 = t17 * t21 * t380;
            let t384 = -t376 + f64x8::splat(0.053425) * t382;
            let t386 = f64x8::splat(0.0621814) * t384 * t52;
            let t387 = t48 * t48;
            let t388 = f64x8::splat(1.0) / t387;
            let t389 = t30 * t388;
            let t390 = f64x8::splat(1.0) / t31;
            let t391 = t375 / f64x8::splat(3.0);
            let t392 = -t391 + t382;
            let t393 = t390 * t392;
            let t395 = f64x8::splat(0.29896666666666666) * t375;
            let t397 = ((t28).sqrt());
            let t398 = t397 * t392;
            let t401 = f64x8::splat(1.0) / t40 / t4;
            let t402 = t401 * t42;
            let t404 = t39 * t402 * t44;
            let t405 = f64x8::splat(0.08215666666666667) * t404;
            let t406 = t26 * t380;
            let t408 = t39 * t43 * t406;
            let t410 = f64x8::splat(1.898925) * t393 - t395 + f64x8::splat(0.8969) * t382 + f64x8::splat(0.3071625) * t398 - t405 + f64x8::splat(0.24647) * t408;
            let t411 = f64x8::splat(1.0) / t51;
            let t412 = t410 * t411;
            let t414 = f64x8::splat(1.0) * t389 * t412;
            let t415 = f64x8::splat(0.017123333333333334) * t375;
            let t417 = -t415 + f64x8::splat(0.05137) * t382;
            let t420 = t72 * t72;
            let t421 = f64x8::splat(1.0) / t420;
            let t422 = t67 * t421;
            let t424 = f64x8::splat(0.516475) * t375;
            let t427 = f64x8::splat(0.104195) * t404;
            let t429 = f64x8::splat(3.529725) * t393 - t424 + f64x8::splat(1.549425) * t382 + f64x8::splat(0.6311625) * t398 - t427 + f64x8::splat(0.312585) * t408;
            let t430 = f64x8::splat(1.0) / t75;
            let t431 = t429 * t430;
            let t434 = f64x8::splat(0.009270833333333334) * t375;
            let t436 = -t434 + f64x8::splat(0.0278125) * t382;
            let t437 = t436 * t89;
            let t439 = t85 * t85;
            let t440 = f64x8::splat(1.0) / t439;
            let t441 = t80 * t440;
            let t443 = f64x8::splat(0.301925) * t375;
            let t446 = f64x8::splat(0.082785) * t404;
            let t448 = f64x8::splat(2.58925) * t393 - t443 + f64x8::splat(0.905775) * t382 + f64x8::splat(0.16504875) * t398 - t446 + f64x8::splat(0.248355) * t408;
            let t449 = f64x8::splat(1.0) / t88;
            let t450 = t448 * t449;
            let t457 = t65 * t80;
            let t459 = t440 * t448 * t449;
            let t462 = -t386 + t414 + t65 * (-f64x8::splat(0.0310907) * t417 * t76 + f64x8::splat(1.0) * t422 * t431 + t386 - t414 - f64x8::splat(0.0197516734986138) * t437 + f64x8::splat(0.5848223622634646) * t441 * t450) + f64x8::splat(0.0197516734986138) * t65 * t437 - f64x8::splat(0.5848223622634646) * t457 * t459;
            let t466 = ((t9).select(f64x8::splat(0.0), t10 * t462 / f64x8::splat(2.0) + t369 * t96 / f64x8::splat(2.0)));
            let t467 = t466 * t152;
            let t468 = t467 * t159;
            let t469 = t100 * t131;
            let t470 = t101 * v_rho0;
            let t472 = f64x8::splat(1.0) / t103 / t470;
            let t473 = v_sigma0 * t472;
            let t475 = v_tau0 * t105;
            let t477 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t473 - f64x8::splat(10.0) / f64x8::splat(3.0) * t475;
            let t478 = param_alpha_ss * t477;
            let t482 = t126 * v_tau0;
            let t485 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t124 * t472 - f64x8::splat(10.0) / f64x8::splat(3.0) * t482 * t105;
            let t487 = t129 * t150;
            let t490 = t136 * t101;
            let t492 = f64x8::splat(1.0) / t102 / t490;
            let t501 = t145 * t127;
            let t504 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t135 * t492 - f64x8::splat(8.0) / f64x8::splat(3.0) * t142 * t472 * t127 - f64x8::splat(10.0) / f64x8::splat(3.0) * t142 * t139 * v_tau0 - f64x8::splat(20.0) / f64x8::splat(3.0) * t501 * t475;
            let t506 = t130 * t130;
            let t507 = f64x8::splat(1.0) / t506;
            let t508 = t148 * t507;
            let t511 = t485 * t131 + t504 * t150 - t469 * t478 - f64x8::splat(2.0) * t487 * t478 - f64x8::splat(3.0) * t508 * t478;
            let t512 = t99 * t511;
            let t513 = t512 * t159;
            let t514 = f64x8::splat(1.0) / t101;
            let t515 = v_sigma0 * t514;
            let t516 = t515 * t156;
            let t517 = t153 * t516;
            let t518 = t517 / f64x8::splat(8.0);
            let t519 = -t368;
            let t520 = ((t163).select(f64x8::splat(0.0), t519));
            let t523 = t17 * t373 * t168;
            let t524 = f64x8::splat(0.017808333333333332) * t523;
            let t525 = f64x8::splat(1.0) / t298;
            let t528 = ((t163).select(f64x8::splat(0.0), -t525 * t519 / f64x8::splat(3.0)));
            let t530 = t17 * t21 * t528;
            let t532 = -t524 + f64x8::splat(0.053425) * t530;
            let t534 = f64x8::splat(0.0621814) * t532 * t186;
            let t535 = t182 * t182;
            let t536 = f64x8::splat(1.0) / t535;
            let t537 = t172 * t536;
            let t538 = f64x8::splat(1.0) / t173;
            let t539 = t523 / f64x8::splat(3.0);
            let t540 = -t539 + t530;
            let t541 = t538 * t540;
            let t543 = f64x8::splat(0.29896666666666666) * t523;
            let t545 = ((t170).sqrt());
            let t546 = t545 * t540;
            let t549 = t39 * t402 * t178;
            let t550 = f64x8::splat(0.08215666666666667) * t549;
            let t551 = t168 * t528;
            let t553 = t39 * t43 * t551;
            let t555 = f64x8::splat(1.898925) * t541 - t543 + f64x8::splat(0.8969) * t530 + f64x8::splat(0.3071625) * t546 - t550 + f64x8::splat(0.24647) * t553;
            let t556 = f64x8::splat(1.0) / t185;
            let t557 = t555 * t556;
            let t559 = f64x8::splat(1.0) * t537 * t557;
            let t560 = f64x8::splat(0.017123333333333334) * t523;
            let t562 = -t560 + f64x8::splat(0.05137) * t530;
            let t565 = t195 * t195;
            let t566 = f64x8::splat(1.0) / t565;
            let t567 = t190 * t566;
            let t569 = f64x8::splat(0.516475) * t523;
            let t572 = f64x8::splat(0.104195) * t549;
            let t574 = f64x8::splat(3.529725) * t541 - t569 + f64x8::splat(1.549425) * t530 + f64x8::splat(0.6311625) * t546 - t572 + f64x8::splat(0.312585) * t553;
            let t575 = f64x8::splat(1.0) / t198;
            let t576 = t574 * t575;
            let t579 = f64x8::splat(0.009270833333333334) * t523;
            let t581 = -t579 + f64x8::splat(0.0278125) * t530;
            let t582 = t581 * t212;
            let t584 = t208 * t208;
            let t585 = f64x8::splat(1.0) / t584;
            let t586 = t203 * t585;
            let t588 = f64x8::splat(0.301925) * t523;
            let t591 = f64x8::splat(0.082785) * t549;
            let t593 = f64x8::splat(2.58925) * t541 - t588 + f64x8::splat(0.905775) * t530 + f64x8::splat(0.16504875) * t546 - t591 + f64x8::splat(0.248355) * t553;
            let t594 = f64x8::splat(1.0) / t211;
            let t595 = t593 * t594;
            let t602 = t65 * t203;
            let t604 = t585 * t593 * t594;
            let t607 = -t534 + t559 + t65 * (-f64x8::splat(0.0310907) * t562 * t199 + f64x8::splat(1.0) * t567 * t576 + t534 - t559 - f64x8::splat(0.0197516734986138) * t582 + f64x8::splat(0.5848223622634646) * t586 * t595) + f64x8::splat(0.0197516734986138) * t65 * t582 - f64x8::splat(0.5848223622634646) * t602 * t604;
            let t611 = ((t164).select(f64x8::splat(0.0), t165 * t607 / f64x8::splat(2.0) + t520 * t219 / f64x8::splat(2.0)));
            let t612 = t611 * t262;
            let t613 = t612 * t269;
            let t614 = t16 * t372;
            let t617 = f64x8::splat(0.0011073470983333333) * t14 * t614 * t287;
            let t618 = t283 * t283;
            let t619 = f64x8::splat(1.0) / t618;
            let t620 = t274 * t619;
            let t622 = f64x8::splat(1.0) / t275 * t11;
            let t623 = t13 * t16;
            let t624 = t623 * t372;
            let t625 = t622 * t624;
            let t627 = t14 * t614;
            let t629 = ((t272).sqrt());
            let t630 = t629 * t11;
            let t631 = t630 * t624;
            let t634 = t38 * t15 * t401;
            let t636 = -f64x8::splat(0.632975) * t625 - f64x8::splat(0.29896666666666666) * t627 - f64x8::splat(0.1023875) * t631 - f64x8::splat(0.08215666666666667) * t634;
            let t637 = f64x8::splat(1.0) / t286;
            let t638 = t636 * t637;
            let t640 = f64x8::splat(1.0) * t620 * t638;
            let t641 = t290 * t3;
            let t642 = t641 * t294;
            let t644 = f64x8::splat(4.0) * t642 * t329;
            let t645 = t293 * t4;
            let t646 = f64x8::splat(1.0) / t645;
            let t647 = t291 * t646;
            let t649 = f64x8::splat(4.0) * t647 * t329;
            let t652 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t368));
            let t655 = ((t163).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t166 * t519));
            let t657 = (t652 + t655) * t64;
            let t658 = t657 * t328;
            let t663 = t308 * t308;
            let t664 = f64x8::splat(1.0) / t663;
            let t665 = t303 * t664;
            let t670 = -f64x8::splat(1.176575) * t625 - f64x8::splat(0.516475) * t627 - f64x8::splat(0.2103875) * t631 - f64x8::splat(0.104195) * t634;
            let t671 = f64x8::splat(1.0) / t311;
            let t672 = t670 * t671;
            let t678 = t321 * t321;
            let t679 = f64x8::splat(1.0) / t678;
            let t680 = t316 * t679;
            let t685 = -f64x8::splat(0.8630833333333333) * t625 - f64x8::splat(0.301925) * t627 - f64x8::splat(0.05501625) * t631 - f64x8::splat(0.082785) * t634;
            let t686 = f64x8::splat(1.0) / t324;
            let t687 = t685 * t686;
            let t690 = f64x8::splat(0.0005323764196666666) * t14 * t614 * t312 + f64x8::splat(1.0) * t665 * t672 - t617 - t640 + f64x8::splat(0.00018311447306006544) * t14 * t614 * t325 + f64x8::splat(0.5848223622634646) * t680 * t687;
            let t691 = t301 * t690;
            let t692 = t295 * t691;
            let t695 = t301 * t11;
            let t697 = t623 * t372 * t325;
            let t699 = f64x8::splat(0.00018311447306006544) * t695 * t697;
            let t700 = t301 * t316;
            let t702 = t679 * t685 * t686;
            let t704 = f64x8::splat(0.5848223622634646) * t700 * t702;
            let t705 = t617 + t640 + t644 - t649 + t295 * t658 + t692 + f64x8::splat(0.0197516734986138) * t657 * t326 - t699 - t704 - t466 - t611;
            let t706 = t705 * t364;
            let t707 = t334 * t349;
            let t708 = param_alpha_ab * t477;
            let t710 = t341 * v_sigma0;
            let t713 = t344 * v_tau0;
            let t716 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t710 * t472 - f64x8::splat(10.0) / f64x8::splat(3.0) * t713 * t105;
            let t718 = t347 * t362;
            let t721 = t351 * t342;
            let t724 = t354 * v_sigma0;
            let t730 = t357 * t345;
            let t733 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t721 * t473 - f64x8::splat(8.0) / f64x8::splat(3.0) * t724 * t472 * t345 - f64x8::splat(10.0) / f64x8::splat(3.0) * t355 * t475 - f64x8::splat(20.0) / f64x8::splat(3.0) * t730 * t475;
            let t735 = t348 * t348;
            let t736 = f64x8::splat(1.0) / t735;
            let t737 = t360 * t736;
            let t740 = t716 * t349 + t733 * t362 - t707 * t708 - f64x8::splat(2.0) * t718 * t708 - f64x8::splat(3.0) * t737 * t708;
            let t741 = t333 * t740;
            let tvrho0 = t160 + t270 + t365 + t4 * (t468 + t513 + t518 + t613 + t706 + t741);
            acc_vrho_0 = tvrho0;
            let t744 = -t5 - t367;
            let t745 = ((t8).select(f64x8::splat(0.0), t744));
            let t749 = ((t8).select(f64x8::splat(0.0), -t377 * t744 / f64x8::splat(3.0)));
            let t751 = t17 * t21 * t749;
            let t753 = -t376 + f64x8::splat(0.053425) * t751;
            let t755 = f64x8::splat(0.0621814) * t753 * t52;
            let t756 = -t391 + t751;
            let t757 = t390 * t756;
            let t760 = t397 * t756;
            let t762 = t26 * t749;
            let t764 = t39 * t43 * t762;
            let t766 = f64x8::splat(1.898925) * t757 - t395 + f64x8::splat(0.8969) * t751 + f64x8::splat(0.3071625) * t760 - t405 + f64x8::splat(0.24647) * t764;
            let t767 = t766 * t411;
            let t769 = f64x8::splat(1.0) * t389 * t767;
            let t771 = -t415 + f64x8::splat(0.05137) * t751;
            let t778 = f64x8::splat(3.529725) * t757 - t424 + f64x8::splat(1.549425) * t751 + f64x8::splat(0.6311625) * t760 - t427 + f64x8::splat(0.312585) * t764;
            let t779 = t778 * t430;
            let t783 = -t434 + f64x8::splat(0.0278125) * t751;
            let t784 = t783 * t89;
            let t790 = f64x8::splat(2.58925) * t757 - t443 + f64x8::splat(0.905775) * t751 + f64x8::splat(0.16504875) * t760 - t446 + f64x8::splat(0.248355) * t764;
            let t791 = t790 * t449;
            let t799 = t440 * t790 * t449;
            let t802 = -t755 + t769 + t65 * (-f64x8::splat(0.0310907) * t771 * t76 + f64x8::splat(1.0) * t422 * t779 + t755 - t769 - f64x8::splat(0.0197516734986138) * t784 + f64x8::splat(0.5848223622634646) * t441 * t791) + f64x8::splat(0.0197516734986138) * t65 * t784 - f64x8::splat(0.5848223622634646) * t457 * t799;
            let t806 = ((t9).select(f64x8::splat(0.0), t10 * t802 / f64x8::splat(2.0) + t745 * t96 / f64x8::splat(2.0)));
            let t807 = t806 * t152;
            let t808 = t807 * t159;
            let t809 = -t744;
            let t810 = ((t163).select(f64x8::splat(0.0), t809));
            let t814 = ((t163).select(f64x8::splat(0.0), -t525 * t809 / f64x8::splat(3.0)));
            let t816 = t17 * t21 * t814;
            let t818 = -t524 + f64x8::splat(0.053425) * t816;
            let t820 = f64x8::splat(0.0621814) * t818 * t186;
            let t821 = -t539 + t816;
            let t822 = t538 * t821;
            let t825 = t545 * t821;
            let t827 = t168 * t814;
            let t829 = t39 * t43 * t827;
            let t831 = f64x8::splat(1.898925) * t822 - t543 + f64x8::splat(0.8969) * t816 + f64x8::splat(0.3071625) * t825 - t550 + f64x8::splat(0.24647) * t829;
            let t832 = t831 * t556;
            let t834 = f64x8::splat(1.0) * t537 * t832;
            let t836 = -t560 + f64x8::splat(0.05137) * t816;
            let t843 = f64x8::splat(3.529725) * t822 - t569 + f64x8::splat(1.549425) * t816 + f64x8::splat(0.6311625) * t825 - t572 + f64x8::splat(0.312585) * t829;
            let t844 = t843 * t575;
            let t848 = -t579 + f64x8::splat(0.0278125) * t816;
            let t849 = t848 * t212;
            let t855 = f64x8::splat(2.58925) * t822 - t588 + f64x8::splat(0.905775) * t816 + f64x8::splat(0.16504875) * t825 - t591 + f64x8::splat(0.248355) * t829;
            let t856 = t855 * t594;
            let t864 = t585 * t855 * t594;
            let t867 = -t820 + t834 + t65 * (-f64x8::splat(0.0310907) * t836 * t199 + f64x8::splat(1.0) * t567 * t844 + t820 - t834 - f64x8::splat(0.0197516734986138) * t849 + f64x8::splat(0.5848223622634646) * t586 * t856) + f64x8::splat(0.0197516734986138) * t65 * t849 - f64x8::splat(0.5848223622634646) * t602 * t864;
            let t871 = ((t164).select(f64x8::splat(0.0), t165 * t867 / f64x8::splat(2.0) + t810 * t219 / f64x8::splat(2.0)));
            let t872 = t871 * t262;
            let t873 = t872 * t269;
            let t874 = t100 * t244;
            let t875 = t223 * v_rho1;
            let t877 = f64x8::splat(1.0) / t225 / t875;
            let t878 = v_sigma2 * t877;
            let t880 = v_tau1 * t227;
            let t882 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t878 - f64x8::splat(10.0) / f64x8::splat(3.0) * t880;
            let t883 = param_alpha_ss * t882;
            let t887 = t126 * v_tau1;
            let t890 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t238 * t877 - f64x8::splat(10.0) / f64x8::splat(3.0) * t887 * t227;
            let t892 = t242 * t260;
            let t895 = t248 * t223;
            let t897 = f64x8::splat(1.0) / t224 / t895;
            let t906 = t145 * t240;
            let t909 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t247 * t897 - f64x8::splat(8.0) / f64x8::splat(3.0) * t253 * t877 * t240 - f64x8::splat(10.0) / f64x8::splat(3.0) * t253 * t251 * v_tau1 - f64x8::splat(20.0) / f64x8::splat(3.0) * t906 * t880;
            let t911 = t243 * t243;
            let t912 = f64x8::splat(1.0) / t911;
            let t913 = t258 * t912;
            let t916 = t890 * t244 + t909 * t260 - t874 * t883 - f64x8::splat(2.0) * t892 * t883 - f64x8::splat(3.0) * t913 * t883;
            let t917 = t222 * t916;
            let t918 = t917 * t269;
            let t919 = f64x8::splat(1.0) / t223;
            let t920 = v_sigma2 * t919;
            let t921 = t920 * t266;
            let t922 = t263 * t921;
            let t923 = t922 / f64x8::splat(8.0);
            let t926 = ((t8).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t744));
            let t929 = ((t163).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t166 * t809));
            let t931 = (t926 + t929) * t64;
            let t932 = t931 * t328;
            let t936 = t617 + t640 - t644 - t649 + t295 * t932 + t692 + f64x8::splat(0.0197516734986138) * t931 * t326 - t699 - t704 - t806 - t871;
            let t937 = t936 * t364;
            let t938 = param_alpha_ab * t882;
            let t940 = t341 * v_sigma2;
            let t943 = t344 * v_tau1;
            let t946 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t940 * t877 - f64x8::splat(10.0) / f64x8::splat(3.0) * t943 * t227;
            let t952 = t354 * v_sigma2;
            let t960 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t721 * t878 - f64x8::splat(8.0) / f64x8::splat(3.0) * t952 * t877 * t345 - f64x8::splat(10.0) / f64x8::splat(3.0) * t355 * t880 - f64x8::splat(20.0) / f64x8::splat(3.0) * t730 * t880;
            let t964 = t946 * t349 + t960 * t362 - t707 * t938 - f64x8::splat(2.0) * t718 * t938 - f64x8::splat(3.0) * t737 * t938;
            let t965 = t333 * t964;
            let tvrho1 = t160 + t270 + t365 + t4 * (t808 + t873 + t918 + t923 + t937 + t965);
            acc_vrho_1 = tvrho1;
            let t968 = param_alpha_ss * t105;
            let t969 = t469 * t968;
            let t970 = t123 * t105;
            let t972 = t487 * t968;
            let t974 = t133 * v_sigma0;
            let t979 = t141 * t105 * t127 + f64x8::splat(2.0) * t974 * t139;
            let t981 = t508 * t968;
            let t983 = t970 * t131 + t979 * t150 - t969 - f64x8::splat(2.0) * t972 - f64x8::splat(3.0) * t981;
            let t984 = t99 * t983;
            let t985 = t984 * t159;
            let t986 = t154 * t156;
            let t988 = t153 * t986 / f64x8::splat(8.0);
            let t989 = param_alpha_ab * t105;
            let t990 = t707 * t989;
            let t991 = t341 * t105;
            let t993 = t718 * t989;
            let t997 = t354 * t105;
            let t999 = f64x8::splat(2.0) * t721 * t105 + t997 * t345;
            let t1001 = t737 * t989;
            let t1003 = t991 * t349 + t999 * t362 - f64x8::splat(3.0) * t1001 - t990 - f64x8::splat(2.0) * t993;
            let t1004 = t333 * t1003;
            let tvsigma0 = t4 * (t985 - t988 + t1004);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t1006 = param_alpha_ss * t227;
            let t1007 = t874 * t1006;
            let t1008 = t123 * t227;
            let t1010 = t892 * t1006;
            let t1012 = t133 * v_sigma2;
            let t1017 = t141 * t227 * t240 + f64x8::splat(2.0) * t1012 * t251;
            let t1019 = t913 * t1006;
            let t1021 = t1008 * t244 + t1017 * t260 - t1007 - f64x8::splat(2.0) * t1010 - f64x8::splat(3.0) * t1019;
            let t1022 = t222 * t1021;
            let t1023 = t1022 * t269;
            let t1024 = t264 * t266;
            let t1026 = t263 * t1024 / f64x8::splat(8.0);
            let t1027 = param_alpha_ab * t227;
            let t1028 = t707 * t1027;
            let t1029 = t341 * t227;
            let t1031 = t718 * t1027;
            let t1035 = t354 * t227;
            let t1037 = t1035 * t345 + f64x8::splat(2.0) * t721 * t227;
            let t1039 = t737 * t1027;
            let t1041 = t1029 * t349 + t1037 * t362 - t1028 - f64x8::splat(2.0) * t1031 - f64x8::splat(3.0) * t1039;
            let t1042 = t333 * t1041;
            let tvsigma2 = t4 * (t1023 - t1026 + t1042);
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t1044 = param_alpha_ss * t108;
            let t1047 = t126 * t108;
            let t1053 = f64x8::splat(1.0) / t102 / t136;
            let t1058 = f64x8::splat(2.0) * t142 * t1053 + f64x8::splat(4.0) * t501 * t108;
            let t1062 = -f64x8::splat(2.0) * t469 * t1044 - f64x8::splat(4.0) * t487 * t1044 - f64x8::splat(6.0) * t508 * t1044 + f64x8::splat(2.0) * t1047 * t131 + t1058 * t150;
            let t1063 = t99 * t1062;
            let t1064 = t1063 * t159;
            let t1065 = v_tau0 * v_tau0;
            let t1066 = f64x8::splat(1.0) / t1065;
            let t1067 = t155 * t1066;
            let t1069 = t153 * t1067 / f64x8::splat(8.0);
            let t1070 = param_alpha_ab * t108;
            let t1073 = t344 * t108;
            let t1082 = f64x8::splat(2.0) * t355 * t108 + f64x8::splat(4.0) * t730 * t108;
            let t1086 = -f64x8::splat(2.0) * t707 * t1070 - f64x8::splat(4.0) * t718 * t1070 - f64x8::splat(6.0) * t737 * t1070 + f64x8::splat(2.0) * t1073 * t349 + t1082 * t362;
            let t1087 = t333 * t1086;
            let tvtau0 = t4 * (t1064 + t1069 + t1087);
            acc_vtau_0 = tvtau0;
            let t1089 = param_alpha_ss * t230;
            let t1092 = t126 * t230;
            let t1098 = f64x8::splat(1.0) / t224 / t248;
            let t1103 = f64x8::splat(2.0) * t253 * t1098 + f64x8::splat(4.0) * t906 * t230;
            let t1107 = -f64x8::splat(2.0) * t874 * t1089 - f64x8::splat(4.0) * t892 * t1089 - f64x8::splat(6.0) * t913 * t1089 + f64x8::splat(2.0) * t1092 * t244 + t1103 * t260;
            let t1108 = t222 * t1107;
            let t1109 = t1108 * t269;
            let t1110 = v_tau1 * v_tau1;
            let t1111 = f64x8::splat(1.0) / t1110;
            let t1112 = t265 * t1111;
            let t1114 = t263 * t1112 / f64x8::splat(8.0);
            let t1115 = param_alpha_ab * t230;
            let t1118 = t344 * t230;
            let t1127 = f64x8::splat(2.0) * t355 * t230 + f64x8::splat(4.0) * t730 * t230;
            let t1131 = -f64x8::splat(2.0) * t707 * t1115 - f64x8::splat(4.0) * t718 * t1115 - f64x8::splat(6.0) * t737 * t1115 + f64x8::splat(2.0) * t1118 * t349 + t1127 * t362;
            let t1132 = t333 * t1131;
            let tvtau1 = t4 * (t1109 + t1114 + t1132);
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
