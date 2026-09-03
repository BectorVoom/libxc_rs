//! GGA_C_BMK vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`
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
pub fn gga_c_bmk_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = f64x8::splat(1.0) + t5;
            let t7 = (t6).simd_le(zeta_threshold);
            let t8 = ((v_rho0).simd_le(dens_threshold)) | (t7);
            let t9 = ((t7).select(zeta_threshold, t6));
            let t10 = f64x8::splat(M_CBRT3);
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = t10 * t12;
            let t14 = f64x8::splat(M_CBRT4);
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = (simd::cbrt(t3));
            let t18 = f64x8::splat(1.0) / t17;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = (simd::cbrt(t6));
            let t25 = ((t7).select(t22, f64x8::splat(1.0) / t23));
            let t27 = t16 * t20 * t25;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t27;
            let t30 = ((t27).sqrt());
            let t33 = ((t27) * (t27).sqrt());
            let t35 = t10 * t10;
            let t36 = t12 * t12;
            let t37 = t35 * t36;
            let t38 = t37 * t14;
            let t39 = t17 * t17;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t19 * t19;
            let t42 = t40 * t41;
            let t43 = t25 * t25;
            let t45 = t38 * t42 * t43;
            let t47 = f64x8::splat(3.79785) * t30 + f64x8::splat(0.8969) * t27 + f64x8::splat(0.204775) * t33 + f64x8::splat(0.123235) * t45;
            let t50 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t47;
            let t51 = (simd::ln(t50));
            let t53 = f64x8::splat(0.062182) * t29 * t51;
            let t55 = t21 * zeta_threshold;
            let t57 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t55, f64x8::splat(2.0) * t19));
            let t59 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t55, f64x8::splat(0.0)));
            let t63 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t19 - f64x8::splat(2.0));
            let t64 = (t57 + t59 - f64x8::splat(2.0)) * t63;
            let t66 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t27;
            let t71 = f64x8::splat(7.05945) * t30 + f64x8::splat(1.549425) * t27 + f64x8::splat(0.420775) * t33 + f64x8::splat(0.1562925) * t45;
            let t74 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t71;
            let t75 = (simd::ln(t74));
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t27;
            let t84 = f64x8::splat(5.1785) * t30 + f64x8::splat(0.905775) * t27 + f64x8::splat(0.1100325) * t33 + f64x8::splat(0.1241775) * t45;
            let t87 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t84;
            let t88 = (simd::ln(t87));
            let t89 = t79 * t88;
            let t95 = -t53 + t64 * (-f64x8::splat(0.03109) * t66 * t75 + t53 - f64x8::splat(0.019751789702565206) * t89) + f64x8::splat(0.019751789702565206) * t64 * t89;
            let t98 = ((t8).select(f64x8::splat(0.0), t9 * t95 / f64x8::splat(2.0)));
            let t99 = param_c_ss_0;
            let t100 = param_c_ss_1;
            let t101 = t100 * v_sigma0;
            let t102 = v_rho0 * v_rho0;
            let t103 = (simd::cbrt(v_rho0));
            let t104 = t103 * t103;
            let t106 = f64x8::splat(1.0) / t104 / t102;
            let t107 = v_sigma0 * t106;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.2) * t107;
            let t110 = f64x8::splat(1.0) / t109;
            let t114 = param_c_ss_2;
            let t115 = v_sigma0 * v_sigma0;
            let t116 = t114 * t115;
            let t117 = t102 * t102;
            let t118 = t117 * v_rho0;
            let t120 = f64x8::splat(1.0) / t103 / t118;
            let t121 = t109 * t109;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t120 * t122;
            let t126 = param_c_ss_3;
            let t127 = t115 * v_sigma0;
            let t128 = t126 * t127;
            let t129 = t117 * t117;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t121 * t109;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = t130 * t132;
            let t136 = param_c_ss_4;
            let t137 = t115 * t115;
            let t138 = t136 * t137;
            let t139 = t129 * t102;
            let t141 = f64x8::splat(1.0) / t104 / t139;
            let t142 = t121 * t121;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t141 * t143;
            let t147 = t99 + f64x8::splat(0.2) * t101 * t106 * t110 + f64x8::splat(0.04) * t116 * t123 + f64x8::splat(0.008) * t128 * t133 + f64x8::splat(0.0016) * t138 * t144;
            let t148 = t98 * t147;
            let t150 = f64x8::splat(1.0) - t5;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = ((v_rho1).simd_le(dens_threshold)) | (t151);
            let t153 = ((t151).select(zeta_threshold, t150));
            let t154 = (simd::cbrt(t150));
            let t156 = ((t151).select(t22, f64x8::splat(1.0) / t154));
            let t158 = t16 * t20 * t156;
            let t160 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t158;
            let t161 = ((t158).sqrt());
            let t164 = ((t158) * (t158).sqrt());
            let t166 = t156 * t156;
            let t168 = t38 * t42 * t166;
            let t170 = f64x8::splat(3.79785) * t161 + f64x8::splat(0.8969) * t158 + f64x8::splat(0.204775) * t164 + f64x8::splat(0.123235) * t168;
            let t173 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t170;
            let t174 = (simd::ln(t173));
            let t176 = f64x8::splat(0.062182) * t160 * t174;
            let t178 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t158;
            let t183 = f64x8::splat(7.05945) * t161 + f64x8::splat(1.549425) * t158 + f64x8::splat(0.420775) * t164 + f64x8::splat(0.1562925) * t168;
            let t186 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t183;
            let t187 = (simd::ln(t186));
            let t191 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t158;
            let t196 = f64x8::splat(5.1785) * t161 + f64x8::splat(0.905775) * t158 + f64x8::splat(0.1100325) * t164 + f64x8::splat(0.1241775) * t168;
            let t199 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t196;
            let t200 = (simd::ln(t199));
            let t201 = t191 * t200;
            let t207 = -t176 + t64 * (-f64x8::splat(0.03109) * t178 * t187 + t176 - f64x8::splat(0.019751789702565206) * t201) + f64x8::splat(0.019751789702565206) * t64 * t201;
            let t210 = ((t152).select(f64x8::splat(0.0), t153 * t207 / f64x8::splat(2.0)));
            let t211 = t100 * v_sigma2;
            let t212 = v_rho1 * v_rho1;
            let t213 = (simd::cbrt(v_rho1));
            let t214 = t213 * t213;
            let t216 = f64x8::splat(1.0) / t214 / t212;
            let t217 = v_sigma2 * t216;
            let t219 = f64x8::splat(1.0) + f64x8::splat(0.2) * t217;
            let t220 = f64x8::splat(1.0) / t219;
            let t224 = v_sigma2 * v_sigma2;
            let t225 = t114 * t224;
            let t226 = t212 * t212;
            let t227 = t226 * v_rho1;
            let t229 = f64x8::splat(1.0) / t213 / t227;
            let t230 = t219 * t219;
            let t231 = f64x8::splat(1.0) / t230;
            let t232 = t229 * t231;
            let t235 = t224 * v_sigma2;
            let t236 = t126 * t235;
            let t237 = t226 * t226;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t230 * t219;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t238 * t240;
            let t244 = t224 * t224;
            let t245 = t136 * t244;
            let t246 = t237 * t212;
            let t248 = f64x8::splat(1.0) / t214 / t246;
            let t249 = t230 * t230;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t248 * t250;
            let t254 = t99 + f64x8::splat(0.2) * t211 * t216 * t220 + f64x8::splat(0.04) * t225 * t232 + f64x8::splat(0.008) * t236 * t241 + f64x8::splat(0.0016) * t245 * t251;
            let t255 = t210 * t254;
            let t257 = t13 * t15 * t18;
            let t259 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t257;
            let t260 = ((t257).sqrt());
            let t263 = ((t257) * (t257).sqrt());
            let t266 = t37 * t14 * t40;
            let t268 = f64x8::splat(3.79785) * t260 + f64x8::splat(0.8969) * t257 + f64x8::splat(0.204775) * t263 + f64x8::splat(0.123235) * t266;
            let t271 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t268;
            let t272 = (simd::ln(t271));
            let t274 = f64x8::splat(0.062182) * t259 * t272;
            let t275 = t2 * t2;
            let t276 = t275 * t275;
            let t277 = t3 * t3;
            let t278 = t277 * t277;
            let t279 = f64x8::splat(1.0) / t278;
            let t280 = t276 * t279;
            let t281 = t23 * t6;
            let t282 = ((t7).select(t55, t281));
            let t283 = t154 * t150;
            let t284 = ((t151).select(t55, t283));
            let t285 = t282 + t284 - f64x8::splat(2.0);
            let t286 = t285 * t63;
            let t288 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t257;
            let t293 = f64x8::splat(7.05945) * t260 + f64x8::splat(1.549425) * t257 + f64x8::splat(0.420775) * t263 + f64x8::splat(0.1562925) * t266;
            let t296 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t293;
            let t297 = (simd::ln(t296));
            let t301 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t257;
            let t306 = f64x8::splat(5.1785) * t260 + f64x8::splat(0.905775) * t257 + f64x8::splat(0.1100325) * t263 + f64x8::splat(0.1241775) * t266;
            let t309 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t306;
            let t310 = (simd::ln(t309));
            let t311 = t301 * t310;
            let t313 = -f64x8::splat(0.03109) * t288 * t297 + t274 - f64x8::splat(0.019751789702565206) * t311;
            let t314 = t286 * t313;
            let t318 = -t274 + t280 * t314 + f64x8::splat(0.019751789702565206) * t286 * t311 - t98 - t210;
            let t320 = param_c_ab_1;
            let t321 = t107 + t217;
            let t322 = t320 * t321;
            let t325 = f64x8::splat(1.0) + f64x8::splat(0.003) * t107 + f64x8::splat(0.003) * t217;
            let t326 = f64x8::splat(1.0) / t325;
            let t329 = param_c_ab_2;
            let t330 = t321 * t321;
            let t331 = t329 * t330;
            let t332 = t325 * t325;
            let t333 = f64x8::splat(1.0) / t332;
            let t336 = param_c_ab_3;
            let t337 = t330 * t321;
            let t338 = t336 * t337;
            let t339 = t332 * t325;
            let t340 = f64x8::splat(1.0) / t339;
            let t343 = param_c_ab_4;
            let t344 = t330 * t330;
            let t345 = t343 * t344;
            let t346 = t332 * t332;
            let t347 = f64x8::splat(1.0) / t346;
            let t350 = param_c_ab_0 + f64x8::splat(0.003) * t322 * t326 + f64x8::splat(9e-06) * t331 * t333 + f64x8::splat(2.7e-08) * t338 * t340 + f64x8::splat(8.1e-11) * t345 * t347;
            let t351 = t318 * t350;
            let tzk0 = t148 + t255 + t351;
            acc_zk = tzk0;
            let t352 = f64x8::splat(1.0) / t277;
            let t353 = t2 * t352;
            let t354 = t4 - t353;
            let t355 = ((t7).select(f64x8::splat(0.0), t354));
            let t358 = f64x8::splat(1.0) / t17 / t3;
            let t359 = t358 * t19;
            let t361 = t16 * t359 * t25;
            let t362 = f64x8::splat(0.017808333333333332) * t361;
            let t363 = f64x8::splat(1.0) / t281;
            let t366 = ((t7).select(f64x8::splat(0.0), -t363 * t354 / f64x8::splat(3.0)));
            let t368 = t16 * t20 * t366;
            let t370 = -t362 + f64x8::splat(0.053425) * t368;
            let t372 = f64x8::splat(0.062182) * t370 * t51;
            let t373 = t47 * t47;
            let t374 = f64x8::splat(1.0) / t373;
            let t375 = t29 * t374;
            let t376 = f64x8::splat(1.0) / t30;
            let t377 = t361 / f64x8::splat(3.0);
            let t378 = -t377 + t368;
            let t379 = t376 * t378;
            let t381 = f64x8::splat(0.29896666666666666) * t361;
            let t383 = ((t27).sqrt());
            let t384 = t383 * t378;
            let t387 = f64x8::splat(1.0) / t39 / t3;
            let t388 = t387 * t41;
            let t390 = t38 * t388 * t43;
            let t391 = f64x8::splat(0.08215666666666667) * t390;
            let t392 = t25 * t366;
            let t394 = t38 * t42 * t392;
            let t396 = f64x8::splat(1.898925) * t379 - t381 + f64x8::splat(0.8969) * t368 + f64x8::splat(0.3071625) * t384 - t391 + f64x8::splat(0.24647) * t394;
            let t397 = f64x8::splat(1.0) / t50;
            let t398 = t396 * t397;
            let t400 = f64x8::splat(1.0) * t375 * t398;
            let t401 = f64x8::splat(0.017123333333333334) * t361;
            let t403 = -t401 + f64x8::splat(0.05137) * t368;
            let t406 = t71 * t71;
            let t407 = f64x8::splat(1.0) / t406;
            let t408 = t66 * t407;
            let t410 = f64x8::splat(0.516475) * t361;
            let t413 = f64x8::splat(0.104195) * t390;
            let t415 = f64x8::splat(3.529725) * t379 - t410 + f64x8::splat(1.549425) * t368 + f64x8::splat(0.6311625) * t384 - t413 + f64x8::splat(0.312585) * t394;
            let t416 = f64x8::splat(1.0) / t74;
            let t417 = t415 * t416;
            let t420 = f64x8::splat(0.009270833333333334) * t361;
            let t422 = -t420 + f64x8::splat(0.0278125) * t368;
            let t423 = t422 * t88;
            let t425 = t84 * t84;
            let t426 = f64x8::splat(1.0) / t425;
            let t427 = t79 * t426;
            let t429 = f64x8::splat(0.301925) * t361;
            let t432 = f64x8::splat(0.082785) * t390;
            let t434 = f64x8::splat(2.58925) * t379 - t429 + f64x8::splat(0.905775) * t368 + f64x8::splat(0.16504875) * t384 - t432 + f64x8::splat(0.248355) * t394;
            let t435 = f64x8::splat(1.0) / t87;
            let t436 = t434 * t435;
            let t443 = t64 * t79;
            let t445 = t426 * t434 * t435;
            let t448 = -t372 + t400 + t64 * (-f64x8::splat(0.03109) * t403 * t75 + f64x8::splat(1.0) * t408 * t417 + t372 - t400 - f64x8::splat(0.019751789702565206) * t423 + f64x8::splat(0.5848223397455204) * t427 * t436) + f64x8::splat(0.019751789702565206) * t64 * t423 - f64x8::splat(0.5848223397455204) * t443 * t445;
            let t452 = ((t8).select(f64x8::splat(0.0), t355 * t95 / f64x8::splat(2.0) + t9 * t448 / f64x8::splat(2.0)));
            let t453 = t452 * t147;
            let t454 = t102 * v_rho0;
            let t456 = f64x8::splat(1.0) / t104 / t454;
            let t460 = t100 * t115;
            let t461 = t117 * t102;
            let t463 = f64x8::splat(1.0) / t103 / t461;
            let t464 = t463 * t122;
            let t469 = t114 * t127;
            let t470 = t129 * v_rho0;
            let t471 = f64x8::splat(1.0) / t470;
            let t472 = t471 * t132;
            let t477 = t126 * t137;
            let t478 = t129 * t454;
            let t480 = f64x8::splat(1.0) / t104 / t478;
            let t481 = t480 * t143;
            let t486 = t137 * v_sigma0;
            let t487 = t136 * t486;
            let t488 = t129 * t461;
            let t492 = f64x8::splat(1.0) / t142 / t109;
            let t493 = f64x8::splat(1.0) / t103 / t488 * t492;
            let t496 = -f64x8::splat(0.5333333333333333) * t101 * t456 * t110 + f64x8::splat(0.10666666666666667) * t460 * t464 - f64x8::splat(0.21333333333333335) * t116 * t464 + f64x8::splat(0.042666666666666665) * t469 * t472 - f64x8::splat(0.064) * t128 * t472 + f64x8::splat(0.0128) * t477 * t481 - f64x8::splat(0.017066666666666667) * t138 * t481 + f64x8::splat(0.0034133333333333333) * t487 * t493;
            let t497 = t98 * t496;
            let t498 = -t354;
            let t499 = ((t151).select(f64x8::splat(0.0), t498));
            let t502 = t16 * t359 * t156;
            let t503 = f64x8::splat(0.017808333333333332) * t502;
            let t504 = f64x8::splat(1.0) / t283;
            let t507 = ((t151).select(f64x8::splat(0.0), -t504 * t498 / f64x8::splat(3.0)));
            let t509 = t16 * t20 * t507;
            let t511 = -t503 + f64x8::splat(0.053425) * t509;
            let t513 = f64x8::splat(0.062182) * t511 * t174;
            let t514 = t170 * t170;
            let t515 = f64x8::splat(1.0) / t514;
            let t516 = t160 * t515;
            let t517 = f64x8::splat(1.0) / t161;
            let t518 = t502 / f64x8::splat(3.0);
            let t519 = -t518 + t509;
            let t520 = t517 * t519;
            let t522 = f64x8::splat(0.29896666666666666) * t502;
            let t524 = ((t158).sqrt());
            let t525 = t524 * t519;
            let t528 = t38 * t388 * t166;
            let t529 = f64x8::splat(0.08215666666666667) * t528;
            let t530 = t156 * t507;
            let t532 = t38 * t42 * t530;
            let t534 = f64x8::splat(1.898925) * t520 - t522 + f64x8::splat(0.8969) * t509 + f64x8::splat(0.3071625) * t525 - t529 + f64x8::splat(0.24647) * t532;
            let t535 = f64x8::splat(1.0) / t173;
            let t536 = t534 * t535;
            let t538 = f64x8::splat(1.0) * t516 * t536;
            let t539 = f64x8::splat(0.017123333333333334) * t502;
            let t541 = -t539 + f64x8::splat(0.05137) * t509;
            let t544 = t183 * t183;
            let t545 = f64x8::splat(1.0) / t544;
            let t546 = t178 * t545;
            let t548 = f64x8::splat(0.516475) * t502;
            let t551 = f64x8::splat(0.104195) * t528;
            let t553 = f64x8::splat(3.529725) * t520 - t548 + f64x8::splat(1.549425) * t509 + f64x8::splat(0.6311625) * t525 - t551 + f64x8::splat(0.312585) * t532;
            let t554 = f64x8::splat(1.0) / t186;
            let t555 = t553 * t554;
            let t558 = f64x8::splat(0.009270833333333334) * t502;
            let t560 = -t558 + f64x8::splat(0.0278125) * t509;
            let t561 = t560 * t200;
            let t563 = t196 * t196;
            let t564 = f64x8::splat(1.0) / t563;
            let t565 = t191 * t564;
            let t567 = f64x8::splat(0.301925) * t502;
            let t570 = f64x8::splat(0.082785) * t528;
            let t572 = f64x8::splat(2.58925) * t520 - t567 + f64x8::splat(0.905775) * t509 + f64x8::splat(0.16504875) * t525 - t570 + f64x8::splat(0.248355) * t532;
            let t573 = f64x8::splat(1.0) / t199;
            let t574 = t572 * t573;
            let t581 = t64 * t191;
            let t583 = t564 * t572 * t573;
            let t586 = -t513 + t538 + t64 * (-f64x8::splat(0.03109) * t541 * t187 + f64x8::splat(1.0) * t546 * t555 + t513 - t538 - f64x8::splat(0.019751789702565206) * t561 + f64x8::splat(0.5848223397455204) * t565 * t574) + f64x8::splat(0.019751789702565206) * t64 * t561 - f64x8::splat(0.5848223397455204) * t581 * t583;
            let t590 = ((t152).select(f64x8::splat(0.0), t153 * t586 / f64x8::splat(2.0) + t499 * t207 / f64x8::splat(2.0)));
            let t591 = t590 * t254;
            let t592 = t15 * t358;
            let t595 = f64x8::splat(0.0011073577833333333) * t13 * t592 * t272;
            let t596 = t268 * t268;
            let t597 = f64x8::splat(1.0) / t596;
            let t598 = t259 * t597;
            let t600 = f64x8::splat(1.0) / t260 * t10;
            let t601 = t12 * t15;
            let t602 = t601 * t358;
            let t603 = t600 * t602;
            let t605 = t13 * t592;
            let t607 = ((t257).sqrt());
            let t608 = t607 * t10;
            let t609 = t608 * t602;
            let t612 = t37 * t14 * t387;
            let t614 = -f64x8::splat(0.632975) * t603 - f64x8::splat(0.29896666666666666) * t605 - f64x8::splat(0.1023875) * t609 - f64x8::splat(0.08215666666666667) * t612;
            let t615 = f64x8::splat(1.0) / t271;
            let t616 = t614 * t615;
            let t618 = f64x8::splat(1.0) * t598 * t616;
            let t619 = t275 * t2;
            let t620 = t619 * t279;
            let t622 = f64x8::splat(4.0) * t620 * t314;
            let t623 = t278 * t3;
            let t624 = f64x8::splat(1.0) / t623;
            let t625 = t276 * t624;
            let t627 = f64x8::splat(4.0) * t625 * t314;
            let t630 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t354));
            let t633 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t498));
            let t635 = (t630 + t633) * t63;
            let t636 = t635 * t313;
            let t641 = t293 * t293;
            let t642 = f64x8::splat(1.0) / t641;
            let t643 = t288 * t642;
            let t648 = -f64x8::splat(1.176575) * t603 - f64x8::splat(0.516475) * t605 - f64x8::splat(0.2103875) * t609 - f64x8::splat(0.104195) * t612;
            let t649 = f64x8::splat(1.0) / t296;
            let t650 = t648 * t649;
            let t656 = t306 * t306;
            let t657 = f64x8::splat(1.0) / t656;
            let t658 = t301 * t657;
            let t663 = -f64x8::splat(0.8630833333333333) * t603 - f64x8::splat(0.301925) * t605 - f64x8::splat(0.05501625) * t609 - f64x8::splat(0.082785) * t612;
            let t664 = f64x8::splat(1.0) / t309;
            let t665 = t663 * t664;
            let t668 = f64x8::splat(0.0005323644333333333) * t13 * t592 * t297 + f64x8::splat(1.0) * t643 * t650 - t595 - t618 + f64x8::splat(0.0001831155503675316) * t13 * t592 * t310 + f64x8::splat(0.5848223397455204) * t658 * t665;
            let t669 = t286 * t668;
            let t670 = t280 * t669;
            let t673 = t286 * t10;
            let t675 = t601 * t358 * t310;
            let t677 = f64x8::splat(0.0001831155503675316) * t673 * t675;
            let t678 = t286 * t301;
            let t680 = t657 * t663 * t664;
            let t682 = f64x8::splat(0.5848223397455204) * t678 * t680;
            let t683 = t595 + t618 + t622 - t627 + t280 * t636 + t670 + f64x8::splat(0.019751789702565206) * t635 * t311 - t677 - t682 - t452 - t590;
            let t684 = t683 * t350;
            let t685 = t320 * v_sigma0;
            let t689 = t333 * v_sigma0;
            let t690 = t689 * t456;
            let t693 = t329 * t321;
            let t696 = t340 * v_sigma0;
            let t697 = t696 * t456;
            let t700 = t336 * t330;
            let t703 = t347 * v_sigma0;
            let t704 = t703 * t456;
            let t707 = t343 * t337;
            let t711 = f64x8::splat(1.0) / t346 / t325;
            let t712 = t711 * v_sigma0;
            let t716 = -f64x8::splat(0.008) * t685 * t456 * t326 + f64x8::splat(2.4e-05) * t322 * t690 - f64x8::splat(4.8e-05) * t693 * t690 + f64x8::splat(1.44e-07) * t331 * t697 - f64x8::splat(2.16e-07) * t700 * t697 + f64x8::splat(6.48e-10) * t338 * t704 - f64x8::splat(8.64e-10) * t707 * t704 + f64x8::splat(2.592e-12) * t345 * t712 * t456;
            let t717 = t318 * t716;
            let tvrho0 = t148 + t255 + t351 + t3 * (t453 + t497 + t591 + t684 + t717);
            acc_vrho_0 = tvrho0;
            let t720 = -t4 - t353;
            let t721 = ((t7).select(f64x8::splat(0.0), t720));
            let t725 = ((t7).select(f64x8::splat(0.0), -t363 * t720 / f64x8::splat(3.0)));
            let t727 = t16 * t20 * t725;
            let t729 = -t362 + f64x8::splat(0.053425) * t727;
            let t731 = f64x8::splat(0.062182) * t729 * t51;
            let t732 = -t377 + t727;
            let t733 = t376 * t732;
            let t736 = t383 * t732;
            let t738 = t25 * t725;
            let t740 = t38 * t42 * t738;
            let t742 = f64x8::splat(1.898925) * t733 - t381 + f64x8::splat(0.8969) * t727 + f64x8::splat(0.3071625) * t736 - t391 + f64x8::splat(0.24647) * t740;
            let t743 = t742 * t397;
            let t745 = f64x8::splat(1.0) * t375 * t743;
            let t747 = -t401 + f64x8::splat(0.05137) * t727;
            let t754 = f64x8::splat(3.529725) * t733 - t410 + f64x8::splat(1.549425) * t727 + f64x8::splat(0.6311625) * t736 - t413 + f64x8::splat(0.312585) * t740;
            let t755 = t754 * t416;
            let t759 = -t420 + f64x8::splat(0.0278125) * t727;
            let t760 = t759 * t88;
            let t766 = f64x8::splat(2.58925) * t733 - t429 + f64x8::splat(0.905775) * t727 + f64x8::splat(0.16504875) * t736 - t432 + f64x8::splat(0.248355) * t740;
            let t767 = t766 * t435;
            let t775 = t426 * t766 * t435;
            let t778 = -t731 + t745 + t64 * (-f64x8::splat(0.03109) * t747 * t75 + f64x8::splat(1.0) * t408 * t755 + t731 - t745 - f64x8::splat(0.019751789702565206) * t760 + f64x8::splat(0.5848223397455204) * t427 * t767) + f64x8::splat(0.019751789702565206) * t64 * t760 - f64x8::splat(0.5848223397455204) * t443 * t775;
            let t782 = ((t8).select(f64x8::splat(0.0), t721 * t95 / f64x8::splat(2.0) + t9 * t778 / f64x8::splat(2.0)));
            let t783 = t782 * t147;
            let t784 = -t720;
            let t785 = ((t151).select(f64x8::splat(0.0), t784));
            let t789 = ((t151).select(f64x8::splat(0.0), -t504 * t784 / f64x8::splat(3.0)));
            let t791 = t16 * t20 * t789;
            let t793 = -t503 + f64x8::splat(0.053425) * t791;
            let t795 = f64x8::splat(0.062182) * t793 * t174;
            let t796 = -t518 + t791;
            let t797 = t517 * t796;
            let t800 = t524 * t796;
            let t802 = t156 * t789;
            let t804 = t38 * t42 * t802;
            let t806 = f64x8::splat(1.898925) * t797 - t522 + f64x8::splat(0.8969) * t791 + f64x8::splat(0.3071625) * t800 - t529 + f64x8::splat(0.24647) * t804;
            let t807 = t806 * t535;
            let t809 = f64x8::splat(1.0) * t516 * t807;
            let t811 = -t539 + f64x8::splat(0.05137) * t791;
            let t818 = f64x8::splat(3.529725) * t797 - t548 + f64x8::splat(1.549425) * t791 + f64x8::splat(0.6311625) * t800 - t551 + f64x8::splat(0.312585) * t804;
            let t819 = t818 * t554;
            let t823 = -t558 + f64x8::splat(0.0278125) * t791;
            let t824 = t823 * t200;
            let t830 = f64x8::splat(2.58925) * t797 - t567 + f64x8::splat(0.905775) * t791 + f64x8::splat(0.16504875) * t800 - t570 + f64x8::splat(0.248355) * t804;
            let t831 = t830 * t573;
            let t839 = t564 * t830 * t573;
            let t842 = -t795 + t809 + t64 * (-f64x8::splat(0.03109) * t811 * t187 + f64x8::splat(1.0) * t546 * t819 + t795 - t809 - f64x8::splat(0.019751789702565206) * t824 + f64x8::splat(0.5848223397455204) * t565 * t831) + f64x8::splat(0.019751789702565206) * t64 * t824 - f64x8::splat(0.5848223397455204) * t581 * t839;
            let t846 = ((t152).select(f64x8::splat(0.0), t153 * t842 / f64x8::splat(2.0) + t785 * t207 / f64x8::splat(2.0)));
            let t847 = t846 * t254;
            let t848 = t212 * v_rho1;
            let t850 = f64x8::splat(1.0) / t214 / t848;
            let t854 = t100 * t224;
            let t855 = t226 * t212;
            let t857 = f64x8::splat(1.0) / t213 / t855;
            let t858 = t857 * t231;
            let t863 = t114 * t235;
            let t864 = t237 * v_rho1;
            let t865 = f64x8::splat(1.0) / t864;
            let t866 = t865 * t240;
            let t871 = t126 * t244;
            let t872 = t237 * t848;
            let t874 = f64x8::splat(1.0) / t214 / t872;
            let t875 = t874 * t250;
            let t880 = t244 * v_sigma2;
            let t881 = t136 * t880;
            let t882 = t237 * t855;
            let t886 = f64x8::splat(1.0) / t249 / t219;
            let t887 = f64x8::splat(1.0) / t213 / t882 * t886;
            let t890 = -f64x8::splat(0.5333333333333333) * t211 * t850 * t220 + f64x8::splat(0.10666666666666667) * t854 * t858 - f64x8::splat(0.21333333333333335) * t225 * t858 + f64x8::splat(0.042666666666666665) * t863 * t866 - f64x8::splat(0.064) * t236 * t866 + f64x8::splat(0.0128) * t871 * t875 - f64x8::splat(0.017066666666666667) * t245 * t875 + f64x8::splat(0.0034133333333333333) * t881 * t887;
            let t891 = t210 * t890;
            let t894 = ((t7).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t720));
            let t897 = ((t151).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t784));
            let t899 = (t894 + t897) * t63;
            let t900 = t899 * t313;
            let t904 = t595 + t618 - t622 - t627 + t280 * t900 + t670 + f64x8::splat(0.019751789702565206) * t899 * t311 - t677 - t682 - t782 - t846;
            let t905 = t904 * t350;
            let t906 = t320 * v_sigma2;
            let t910 = t333 * v_sigma2;
            let t911 = t910 * t850;
            let t916 = t340 * v_sigma2;
            let t917 = t916 * t850;
            let t922 = t347 * v_sigma2;
            let t923 = t922 * t850;
            let t928 = t711 * v_sigma2;
            let t932 = -f64x8::splat(0.008) * t906 * t850 * t326 + f64x8::splat(2.4e-05) * t322 * t911 - f64x8::splat(4.8e-05) * t693 * t911 + f64x8::splat(1.44e-07) * t331 * t917 - f64x8::splat(2.16e-07) * t700 * t917 + f64x8::splat(6.48e-10) * t338 * t923 - f64x8::splat(8.64e-10) * t707 * t923 + f64x8::splat(2.592e-12) * t345 * t928 * t850;
            let t933 = t318 * t932;
            let tvrho1 = t148 + t255 + t351 + t3 * (t783 + t847 + t891 + t905 + t933);
            acc_vrho_1 = tvrho1;
            let t941 = t114 * v_sigma0;
            let t946 = t126 * t115;
            let t951 = t136 * t127;
            let t954 = t129 * t118;
            let t957 = f64x8::splat(1.0) / t103 / t954 * t492;
            let t960 = f64x8::splat(0.2) * t100 * t106 * t110 - f64x8::splat(0.04) * t101 * t123 + f64x8::splat(0.08) * t941 * t123 - f64x8::splat(0.016) * t116 * t133 + f64x8::splat(0.024) * t946 * t133 - f64x8::splat(0.0048) * t128 * t144 + f64x8::splat(0.0064) * t951 * t144 - f64x8::splat(0.00128) * t138 * t957;
            let t961 = t98 * t960;
            let t962 = t320 * t106;
            let t965 = t333 * t106;
            let t970 = t340 * t106;
            let t975 = t347 * t106;
            let t983 = f64x8::splat(0.003) * t962 * t326 - f64x8::splat(9e-06) * t322 * t965 + f64x8::splat(1.8e-05) * t693 * t965 - f64x8::splat(5.4e-08) * t331 * t970 + f64x8::splat(8.1e-08) * t700 * t970 - f64x8::splat(2.43e-10) * t338 * t975 + f64x8::splat(3.24e-10) * t707 * t975 - f64x8::splat(9.72e-13) * t345 * t711 * t106;
            let t984 = t318 * t983;
            let tvsigma0 = t3 * (t961 + t984);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t991 = t114 * v_sigma2;
            let t996 = t126 * t224;
            let t1001 = t136 * t235;
            let t1004 = t237 * t227;
            let t1007 = f64x8::splat(1.0) / t213 / t1004 * t886;
            let t1010 = f64x8::splat(0.2) * t100 * t216 * t220 - f64x8::splat(0.04) * t211 * t232 + f64x8::splat(0.08) * t991 * t232 - f64x8::splat(0.016) * t225 * t241 + f64x8::splat(0.024) * t996 * t241 - f64x8::splat(0.0048) * t236 * t251 + f64x8::splat(0.0064) * t1001 * t251 - f64x8::splat(0.00128) * t245 * t1007;
            let t1011 = t210 * t1010;
            let t1012 = t320 * t216;
            let t1015 = t333 * t216;
            let t1020 = t340 * t216;
            let t1025 = t347 * t216;
            let t1030 = t711 * t216;
            let t1033 = f64x8::splat(0.003) * t1012 * t326 - f64x8::splat(9e-06) * t322 * t1015 + f64x8::splat(1.8e-05) * t693 * t1015 - f64x8::splat(5.4e-08) * t331 * t1020 + f64x8::splat(8.1e-08) * t700 * t1020 - f64x8::splat(2.43e-10) * t338 * t1025 + f64x8::splat(3.24e-10) * t707 * t1025 - f64x8::splat(9.72e-13) * t345 * t1030;
            let t1034 = t318 * t1033;
            let tvsigma2 = t3 * (t1011 + t1034);
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
