//! GGA_XC_TH3 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th3.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th3_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rmath::pow(2.0, 1.0 / 6.0);
        let t3 = t2 * t2;
        let t4 = t3 * t3;
        let t6 = param_omega_0 * t4 * t2;
        let t7 = rmath::pow(rho[ip], 1.0 / 6.0);
        let t8 = t7 * rho[ip];
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = param_omega_1 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = t15 * rho[ip];
        let t20 = M_SQRT2;
        let t21 = param_omega_2 * t20;
        let t22 = rmath::sqrt(rho[ip]);
        let t23 = t22 * rho[ip];
        let t27 = param_omega_3 * t12;
        let t28 = t15 * t15;
        let t29 = t28 * rho[ip];
        let t33 = rmath::pow(2.0, 1.0 / 12.0);
        let t34 = t33 * t33;
        let t36 = t34 * t34;
        let t38 = param_omega_4 * t36 * t34 * t33;
        let t39 = rmath::pow(rho[ip], 1.0 / 12.0);
        let t40 = rmath::sqrt(sigma[ip]);
        let t43 = pow_1_3(zeta_threshold);
        let t45 = piecewise3(1.0 <= zeta_threshold, t43 * zeta_threshold, 1.0);
        let t50 = param_omega_5 * t20;
        let t56 = param_omega_6 * t12;
        let t62 = param_omega_7 * t2;
        let t68 = param_omega_8 * t12;
        let t69 = 1.0 / rho[ip];
        let t71 = t45 * t45;
        let t76 = param_omega_9 * t2;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = t78 * t7;
        let t80 = 1.0 / t79;
        let t85 = param_omega_10;
        let t86 = 1.0 / t28;
        let t87 = t85 * t86;
        let t88 = sigma[ip] * t71;
        let t92 = param_omega_11 * t12;
        let t93 = rho[ip] * rho[ip];
        let t95 = 1.0 / t28 / t93;
        let t96 = sigma[ip] * t95;
        let t98 = t96 * t71 - t96;
        let t103 = param_omega_12 * t2;
        let t104 = t79 * rho[ip];
        let t108 = param_omega_13;
        let t109 = t108 * t93;
        let t112 = param_omega_18;
        let t113 = rmath::pow(rho[ip], 1.0833333333333333);
        let t116 = t6 * t8 / 2.0 + t14 * t16 / 2.0 + t21 * t23 / 2.0 + t27 * t29 / 2.0 + t38 * t39 * t40 * t45 / 4.0 + t50 * t7 * t40 * t45 / 4.0 + t56 * t15 * t40 * t45 / 4.0 + t62 * t22 * t40 * t45 / 4.0 + t68 * t69 * sigma[ip] * t71 / 8.0 + t76 * t80 * sigma[ip] * t71 / 8.0 + t87 * t88 / 8.0 + t92 * t29 * t98 / 2.0 + t103 * t104 * t98 / 2.0 + t109 * t98 / 2.0 + 0.9438743126816935 * t112 * t113;
        let tzk0 = t116 * t69;
        zk[ip] += tzk0;
        let t125 = t39 * t39;
        let t127 = t125 * t125;
        let t128 = t127 * t127;
        let t129 = t128 * t125 * t39;
        let t130 = 1.0 / t129;
        let t143 = 1.0 / t22;
        let t148 = 1.0 / t93;
        let t153 = 1.0 / t104;
        let t158 = 1.0 / t29;
        let t159 = t85 * t158;
        let t165 = t93 * rho[ip];
        let t167 = 1.0 / t28 / t165;
        let t168 = sigma[ip] * t167;
        let t171 = -8.0 / 3.0 * t168 * t71 + 8.0 / 3.0 * t168;
        let t181 = t108 * rho[ip];
        let t185 = rmath::pow(rho[ip], 0.08333333333333333);
        let tvrho0 = 7.0 / 12.0 * t6 * t7 + 2.0 / 3.0 * t14 * t15 + 3.0 / 4.0 * t21 * t22 + 5.0 / 6.0 * t27 * t28 + t38 * t130 * t40 * t45 / 48.0 + t50 * t80 * t40 * t45 / 24.0 + t56 * t86 * t40 * t45 / 12.0 + t62 * t143 * t40 * t45 / 8.0 - t68 * t148 * sigma[ip] * t71 / 8.0 - 5.0 / 48.0 * t76 * t153 * sigma[ip] * t71 - t159 * t88 / 12.0 + 5.0 / 6.0 * t92 * t28 * t98 + t92 * t29 * t171 / 2.0 + 11.0 / 12.0 * t103 * t79 * t98 + t103 * t104 * t171 / 2.0 + t181 * t98 + t109 * t171 / 2.0 + 1.0225305054051679 * t112 * t185;
        vrho[ip] += tvrho0;
        let t188 = 1.0 / t40;
        let t214 = t95 * t71 - t95;
        let tvsigma0 = t38 * t39 * t188 * t45 / 8.0 + t50 * t7 * t188 * t45 / 8.0 + t56 * t15 * t188 * t45 / 8.0 + t62 * t22 * t188 * t45 / 8.0 + t68 * t69 * t71 / 8.0 + t76 * t80 * t71 / 8.0 + t87 * t71 / 8.0 + t92 * t29 * t214 / 2.0 + t103 * t104 * t214 / 2.0 + t109 * t214 / 2.0;
        vsigma[ip] += tvsigma0;
        let t229 = 1.0 / t15;
        let t233 = 1.0 / t129 / rho[ip];
        let t246 = 1.0 / t23;
        let t251 = 1.0 / t165;
        let t257 = 1.0 / t79 / t93;
        let t263 = t85 * t95;
        let t272 = t93 * t93;
        let t274 = 1.0 / t28 / t272;
        let t275 = sigma[ip] * t274;
        let t278 = 88.0 / 9.0 * t275 * t71 - 88.0 / 9.0 * t275;
        let t282 = 1.0 / t7;
        let t297 = rmath::pow(rho[ip], -0.9166666666666666);
        let t300 = 5.0 / 36.0 * t263 * t88 + 5.0 / 9.0 * t92 * t229 * t98 + 5.0 / 3.0 * t92 * t28 * t171 + t92 * t29 * t278 / 2.0 + 55.0 / 72.0 * t103 * t282 * t98 + 11.0 / 6.0 * t103 * t79 * t171 + t103 * t104 * t278 / 2.0 + t108 * t98 + 2.0 * t181 * t171 + t109 * t278 / 2.0 + 0.08521087545043066 * t112 * t297;
        let tv2rho20 = 7.0 / 72.0 * t6 * t80 + 2.0 / 9.0 * t14 * t86 + 3.0 / 8.0 * t21 * t143 + 5.0 / 9.0 * t27 * t229 - 11.0 / 576.0 * t38 * t233 * t40 * t45 - 5.0 / 144.0 * t50 * t153 * t40 * t45 - t56 * t158 * t40 * t45 / 18.0 - t62 * t246 * t40 * t45 / 16.0 + t68 * t251 * sigma[ip] * t71 / 4.0 + 55.0 / 288.0 * t76 * t257 * sigma[ip] * t71 + t300;
        v2rho2[ip] += tv2rho20;
        let t330 = -8.0 / 3.0 * t167 * t71 + 8.0 / 3.0 * t167;
        let tv2rhosigma0 = t38 * t130 * t188 * t45 / 96.0 + t50 * t80 * t188 * t45 / 48.0 + t56 * t86 * t188 * t45 / 24.0 + t62 * t143 * t188 * t45 / 16.0 - t68 * t148 * t71 / 8.0 - 5.0 / 48.0 * t76 * t153 * t71 - t159 * t71 / 12.0 + 5.0 / 6.0 * t92 * t28 * t214 + t92 * t29 * t330 / 2.0 + 11.0 / 12.0 * t103 * t79 * t214 + t103 * t104 * t330 / 2.0 + t181 * t214 + t109 * t330 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t344 = 1.0 / t40 / sigma[ip];
        let tv2sigma20 = -t56 * t15 * t344 * t45 / 16.0 - t62 * t22 * t344 * t45 / 16.0 - t38 * t39 * t344 * t45 / 16.0 - t50 * t7 * t344 * t45 / 16.0;
        v2sigma2[ip] += tv2sigma20;
        let t360 = t272 * rho[ip];
        let t362 = 1.0 / t28 / t360;
        let t363 = sigma[ip] * t362;
        let t366 = -1232.0 / 27.0 * t363 * t71 + 1232.0 / 27.0 * t363;
        let t375 = 1.0 / t16;
        let t379 = 1.0 / t129 / t93;
        let t393 = 1.0 / t22 / t93;
        let t398 = 1.0 / t272;
        let t403 = 3.0 * t181 * t278 + t109 * t366 / 2.0 - 35.0 / 432.0 * t6 * t153 - 4.0 / 27.0 * t14 * t158 - 3.0 / 16.0 * t21 * t246 - 5.0 / 27.0 * t27 * t375 + 253.0 / 6912.0 * t38 * t379 * t40 * t45 + 55.0 / 864.0 * t50 * t257 * t40 * t45 + 5.0 / 54.0 * t56 * t95 * t40 * t45 + 3.0 / 32.0 * t62 * t393 * t40 * t45 - 3.0 / 4.0 * t68 * t398 * sigma[ip] * t71;
        let t405 = 1.0 / t79 / t165;
        let t413 = 1.0 / t8;
        let t429 = t85 * t167;
        let t440 = rmath::pow(rho[ip], -1.9166666666666667);
        let t443 = -935.0 / 1728.0 * t76 * t405 * sigma[ip] * t71 - 5.0 / 27.0 * t92 * t375 * t98 - 55.0 / 432.0 * t103 * t413 * t98 + t92 * t29 * t366 / 2.0 + 55.0 / 24.0 * t103 * t282 * t171 + 11.0 / 4.0 * t103 * t79 * t278 + t103 * t104 * t366 / 2.0 - 10.0 / 27.0 * t429 * t88 + 5.0 / 3.0 * t92 * t229 * t171 + 5.0 / 2.0 * t92 * t28 * t278 + 3.0 * t108 * t171 - 0.07810996916289477 * t112 * t440;
        let tv3rho30 = t403 + t443;
        v3rho3[ip] += tv3rho30;
        let t476 = 88.0 / 9.0 * t274 * t71 - 88.0 / 9.0 * t274;
        let tv3rho2sigma0 = -11.0 / 1152.0 * t38 * t233 * t188 * t45 - 5.0 / 288.0 * t50 * t153 * t188 * t45 - t56 * t158 * t188 * t45 / 36.0 - t62 * t246 * t188 * t45 / 32.0 + t68 * t251 * t71 / 4.0 + 55.0 / 288.0 * t76 * t257 * t71 + 5.0 / 36.0 * t263 * t71 + 5.0 / 9.0 * t92 * t229 * t214 + 5.0 / 3.0 * t92 * t28 * t330 + t92 * t29 * t476 / 2.0 + 55.0 / 72.0 * t103 * t282 * t214 + 11.0 / 6.0 * t103 * t79 * t330 + t103 * t104 * t476 / 2.0 + t108 * t214 + 2.0 * t181 * t330 + t109 * t476 / 2.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = -t38 * t130 * t344 * t45 / 192.0 - t50 * t80 * t344 * t45 / 96.0 - t56 * t86 * t344 * t45 / 48.0 - t62 * t143 * t344 * t45 / 32.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t510 = sigma[ip] * sigma[ip];
        let t512 = 1.0 / t40 / t510;
        let tv3sigma30 = 3.0 / 32.0 * t56 * t15 * t512 * t45 + 3.0 / 32.0 * t62 * t22 * t512 * t45 + 3.0 / 32.0 * t38 * t39 * t512 * t45 + 3.0 / 32.0 * t50 * t7 * t512 * t45;
        v3sigma3[ip] += tv3sigma30;
        let t531 = sigma[ip] / t28 / t272 / t93;
        let t534 = 20944.0 / 81.0 * t531 * t71 - 20944.0 / 81.0 * t531;
        let t544 = 1.0 / t15 / t93;
        let t549 = rmath::pow(rho[ip], -2.9166666666666665);
        let t572 = 4.0 * t181 * t366 + t109 * t534 / 2.0 + 385.0 / 2592.0 * t6 * t257 + 20.0 / 81.0 * t14 * t95 + 9.0 / 32.0 * t21 * t393 + 20.0 / 81.0 * t27 * t544 + 6.0 * t108 * t278 + 0.14971077422888165 * t112 * t549 - 8855.0 / 82944.0 * t38 / t129 / t165 * t40 * t45 - 935.0 / 5184.0 * t50 * t405 * t40 * t45 - 20.0 / 81.0 * t56 * t167 * t40 * t45 - 15.0 / 64.0 * t62 / t22 / t165 * t40 * t45;
        let t619 = 3.0 * t68 / t360 * sigma[ip] * t71 + 21505.0 / 10368.0 * t76 / t79 / t272 * sigma[ip] * t71 + 20.0 / 81.0 * t92 * t544 * t98 + 385.0 / 2592.0 * t103 / t7 / t93 * t98 + 10.0 / 3.0 * t92 * t28 * t366 - 20.0 / 27.0 * t92 * t375 * t171 - 55.0 / 108.0 * t103 * t413 * t171 + t92 * t29 * t534 / 2.0 + 55.0 / 12.0 * t103 * t282 * t278 + 11.0 / 3.0 * t103 * t79 * t366 + t103 * t104 * t534 / 2.0 + 110.0 / 81.0 * t85 * t274 * t88 + 10.0 / 3.0 * t92 * t229 * t278;
        let tv4rho40 = t572 + t619;
        v4rho4[ip] += tv4rho40;
        let t655 = -1232.0 / 27.0 * t362 * t71 + 1232.0 / 27.0 * t362;
        let tv4rho3sigma0 = 253.0 / 13824.0 * t38 * t379 * t188 * t45 + 55.0 / 1728.0 * t50 * t257 * t188 * t45 + 5.0 / 108.0 * t56 * t95 * t188 * t45 + 3.0 / 64.0 * t62 * t393 * t188 * t45 - 3.0 / 4.0 * t68 * t398 * t71 - 935.0 / 1728.0 * t76 * t405 * t71 - 10.0 / 27.0 * t429 * t71 - 5.0 / 27.0 * t92 * t375 * t214 + 5.0 / 3.0 * t92 * t229 * t330 + 5.0 / 2.0 * t92 * t28 * t476 + t92 * t29 * t655 / 2.0 - 55.0 / 432.0 * t103 * t413 * t214 + 55.0 / 24.0 * t103 * t282 * t330 + 11.0 / 4.0 * t103 * t79 * t476 + t103 * t104 * t655 / 2.0 + 3.0 * t108 * t330 + 3.0 * t181 * t476 + t109 * t655 / 2.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho2sigma20 = 11.0 / 2304.0 * t38 * t233 * t344 * t45 + 5.0 / 576.0 * t50 * t153 * t344 * t45 + t56 * t158 * t344 * t45 / 72.0 + t62 * t246 * t344 * t45 / 64.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = t38 * t130 * t512 * t45 / 128.0 + t50 * t80 * t512 * t45 / 64.0 + t56 * t86 * t512 * t45 / 32.0 + 3.0 / 64.0 * t62 * t143 * t512 * t45;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t711 = 1.0 / t40 / t510 / sigma[ip];
        let tv4sigma40 = -15.0 / 64.0 * t56 * t15 * t711 * t45 - 15.0 / 64.0 * t62 * t22 * t711 * t45 - 15.0 / 64.0 * t38 * t39 * t711 * t45 - 15.0 / 64.0 * t50 * t7 * t711 * t45;
        v4sigma4[ip] += tv4sigma40;
    }
}
