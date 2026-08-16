//! GGA_X_LB kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_vxc/gga_x_lb.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lb_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_alpha: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t9 = param_alpha * t1 * t4 * t6 / 2.0;
        let t10 = f64::sqrt(sigma0);
        let t11 = pow_1_3(rho0);
        let t13 = 1.0 / t11 / rho0;
        let t14 = t10 * t13;
        let t15 = t14 < 300.0;
        let t16 = param_beta * sigma0;
        let t17 = rho0 * rho0;
        let t18 = t11 * t11;
        let t20 = 1.0 / t18 / t17;
        let t21 = param_beta * t10;
        let t23 = param_gamma * t10 * t13;
        let t24 = f64::ln(t23 + f64::sqrt(t23 * t23 + 1.0));
        let t25 = t13 * t24;
        let t28 = 3.0 * t21 * t25 + 1.0;
        let t29 = 1.0 / t28;
        let t33 = f64::ln(2.0 * t23);
        let t34 = 1.0 / t33;
        let t37 = piecewise3(t15, t16 * t20 * t29, t14 * t34 / 3.0);
        let t38 = -t9 - t37;
        let tvrho0 = t38 * t11;
        vrho[ip * 2] += tvrho0;
        let t39 = f64::sqrt(sigma2);
        let t40 = pow_1_3(rho1);
        let t42 = 1.0 / t40 / rho1;
        let t43 = t39 * t42;
        let t44 = t43 < 300.0;
        let t45 = param_beta * sigma2;
        let t46 = rho1 * rho1;
        let t47 = t40 * t40;
        let t49 = 1.0 / t47 / t46;
        let t50 = param_beta * t39;
        let t52 = param_gamma * t39 * t42;
        let t53 = f64::ln(t52 + f64::sqrt(t52 * t52 + 1.0));
        let t54 = t42 * t53;
        let t57 = 3.0 * t50 * t54 + 1.0;
        let t58 = 1.0 / t57;
        let t62 = f64::ln(2.0 * t52);
        let t63 = 1.0 / t62;
        let t66 = piecewise3(t44, t45 * t49 * t58, t43 * t63 / 3.0);
        let t67 = -t9 - t66;
        let tvrho1 = t67 * t40;
        vrho[ip * 2 + 1] += tvrho1;
        let t68 = t17 * rho0;
        let t70 = 1.0 / t18 / t68;
        let t74 = t28 * t28;
        let t75 = 1.0 / t74;
        let t76 = t20 * t75;
        let t78 = 1.0 / t11 / t17;
        let t79 = t78 * t24;
        let t82 = param_gamma * param_gamma;
        let t85 = t82 * sigma0 * t20 + 1.0;
        let t86 = f64::sqrt(t85);
        let t87 = 1.0 / t86;
        let t88 = t70 * param_gamma * t87;
        let t91 = -4.0 * t16 * t88 - 4.0 * t21 * t79;
        let t95 = t10 * t78;
        let t97 = t33 * t33;
        let t98 = 1.0 / t97;
        let t102 = piecewise3(t15, -8.0 / 3.0 * t16 * t70 * t29 - t16 * t76 * t91, -4.0 / 9.0 * t95 * t34 + 4.0 / 9.0 * t95 * t98);
        let t104 = 1.0 / t18;
        let tv2rho20 = -t102 * t11 + t38 * t104 / 3.0;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = 0.0;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t107 = t46 * rho1;
        let t109 = 1.0 / t47 / t107;
        let t113 = t57 * t57;
        let t114 = 1.0 / t113;
        let t115 = t49 * t114;
        let t117 = 1.0 / t40 / t46;
        let t118 = t117 * t53;
        let t123 = t82 * sigma2 * t49 + 1.0;
        let t124 = f64::sqrt(t123);
        let t125 = 1.0 / t124;
        let t126 = t109 * param_gamma * t125;
        let t129 = -4.0 * t50 * t118 - 4.0 * t45 * t126;
        let t133 = t39 * t117;
        let t135 = t62 * t62;
        let t136 = 1.0 / t135;
        let t140 = piecewise3(t44, -8.0 / 3.0 * t45 * t109 * t58 - t45 * t115 * t129, 4.0 / 9.0 * t133 * t136 - 4.0 / 9.0 * t133 * t63);
        let t142 = 1.0 / t47;
        let tv2rho22 = -t140 * t40 + t67 * t142 / 3.0;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t145 = param_beta * t20;
        let t147 = 1.0 / t10;
        let t148 = param_beta * t147;
        let t150 = param_gamma * t87;
        let t153 = 3.0 / 2.0 * t145 * t150 + 3.0 / 2.0 * t148 * t25;
        let t157 = t147 * t13;
        let t162 = piecewise3(t15, -t16 * t76 * t153 + t145 * t29, t157 * t34 / 6.0 - t157 * t98 / 6.0);
        let tv2rhosigma0 = -t162 * t11;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let tv2rhosigma2 = 0.0;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let tv2rhosigma3 = 0.0;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t164 = param_beta * t49;
        let t166 = 1.0 / t39;
        let t167 = param_beta * t166;
        let t169 = param_gamma * t125;
        let t172 = 3.0 / 2.0 * t164 * t169 + 3.0 / 2.0 * t167 * t54;
        let t176 = t166 * t42;
        let t181 = piecewise3(t44, -t45 * t115 * t172 + t164 * t58, -t176 * t136 / 6.0 + t176 * t63 / 6.0);
        let tv2rhosigma5 = -t181 * t40;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t183 = param_beta * t70;
        let t186 = t75 * t91;
        let t188 = t70 * t75;
        let t192 = t16 * t20;
        let t194 = 1.0 / t74 / t28;
        let t195 = t194 * t153;
        let t196 = t195 * t91;
        let t203 = t17 * t17;
        let t207 = param_beta / t11 / t203 / t17;
        let t208 = t82 * param_gamma;
        let t210 = 1.0 / t86 / t85;
        let t211 = t208 * t210;
        let t212 = t211 * sigma0;
        let t215 = -2.0 * t148 * t79 - 6.0 * t183 * t150 + 2.0 * t207 * t212;
        let t219 = t147 * t78;
        let t225 = 1.0 / t97 / t33;
        let t229 = piecewise3(t15, -8.0 / 3.0 * t183 * t29 - t145 * t186 + 8.0 / 3.0 * t16 * t188 * t153 + 2.0 * t192 * t196 - t16 * t76 * t215, -2.0 / 9.0 * t219 * t34 + 4.0 / 9.0 * t219 * t98 - 4.0 / 9.0 * t219 * t225);
        let tv3rho2sigma0 = -t229 * t11 - t162 * t104 / 3.0;
        v3rho2sigma[ip * 9] += tv3rho2sigma0;
        let tv3rho2sigma1 = 0.0;
        v3rho2sigma[ip * 9 + 1] += tv3rho2sigma1;
        let tv3rho2sigma2 = 0.0;
        v3rho2sigma[ip * 9 + 2] += tv3rho2sigma2;
        let tv3rho2sigma3 = 0.0;
        v3rho2sigma[ip * 9 + 3] += tv3rho2sigma3;
        let tv3rho2sigma4 = 0.0;
        v3rho2sigma[ip * 9 + 4] += tv3rho2sigma4;
        let tv3rho2sigma5 = 0.0;
        v3rho2sigma[ip * 9 + 5] += tv3rho2sigma5;
        let tv3rho2sigma6 = 0.0;
        v3rho2sigma[ip * 9 + 6] += tv3rho2sigma6;
        let tv3rho2sigma7 = 0.0;
        v3rho2sigma[ip * 9 + 7] += tv3rho2sigma7;
        let t233 = param_beta * t109;
        let t236 = t114 * t129;
        let t238 = t109 * t114;
        let t242 = t45 * t49;
        let t244 = 1.0 / t113 / t57;
        let t245 = t244 * t172;
        let t246 = t245 * t129;
        let t253 = t46 * t46;
        let t257 = param_beta / t40 / t253 / t46;
        let t259 = 1.0 / t124 / t123;
        let t260 = t208 * t259;
        let t261 = t260 * sigma2;
        let t264 = -2.0 * t167 * t118 - 6.0 * t233 * t169 + 2.0 * t257 * t261;
        let t268 = t166 * t117;
        let t274 = 1.0 / t135 / t62;
        let t278 = piecewise3(t44, -8.0 / 3.0 * t233 * t58 - t164 * t236 + 8.0 / 3.0 * t45 * t238 * t172 + 2.0 * t242 * t246 - t45 * t115 * t264, -2.0 / 9.0 * t268 * t63 + 4.0 / 9.0 * t268 * t136 - 4.0 / 9.0 * t268 * t274);
        let tv3rho2sigma8 = -t278 * t40 - t181 * t142 / 3.0;
        v3rho2sigma[ip * 9 + 8] += tv3rho2sigma8;
        let t283 = 1.0 / t18 / t203;
        let t290 = t20 * t194;
        let t291 = t91 * t91;
        let t296 = 1.0 / t11 / t68;
        let t297 = t296 * t24;
        let t304 = sigma0 * sigma0;
        let t305 = param_beta * t304;
        let t308 = 1.0 / t11 / t203 / t68;
        let t313 = 28.0 / 3.0 * t21 * t297 + 20.0 * t16 * t283 * param_gamma * t87 - 16.0 / 3.0 * t305 * t308 * t208 * t210;
        let t317 = t10 * t296;
        let t325 = piecewise3(t15, 88.0 / 9.0 * t16 * t283 * t29 + 16.0 / 3.0 * t16 * t188 * t91 + 2.0 * t16 * t290 * t291 - t16 * t76 * t313, 28.0 / 27.0 * t317 * t34 - 44.0 / 27.0 * t317 * t98 + 32.0 / 27.0 * t317 * t225);
        let t330 = 1.0 / t18 / rho0;
        let tv3rho30 = -t325 * t11 - 2.0 / 3.0 * t102 * t104 - 2.0 / 9.0 * t38 * t330;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = 0.0;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = 0.0;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t334 = 1.0 / t47 / t253;
        let t341 = t49 * t244;
        let t342 = t129 * t129;
        let t347 = 1.0 / t40 / t107;
        let t348 = t347 * t53;
        let t355 = sigma2 * sigma2;
        let t356 = param_beta * t355;
        let t359 = 1.0 / t40 / t253 / t107;
        let t364 = 28.0 / 3.0 * t50 * t348 + 20.0 * t45 * t334 * param_gamma * t125 - 16.0 / 3.0 * t356 * t359 * t208 * t259;
        let t368 = t39 * t347;
        let t376 = piecewise3(t44, 88.0 / 9.0 * t45 * t334 * t58 + 16.0 / 3.0 * t45 * t238 * t129 + 2.0 * t45 * t341 * t342 - t45 * t115 * t364, 28.0 / 27.0 * t368 * t63 - 44.0 / 27.0 * t368 * t136 + 32.0 / 27.0 * t368 * t274);
        let t381 = 1.0 / t47 / rho1;
        let tv3rho33 = -t376 * t40 - 2.0 / 3.0 * t140 * t142 - 2.0 / 9.0 * t67 * t381;
        v3rho3[ip * 4 + 3] += tv3rho33;
        let t384 = t75 * t153;
        let t387 = t153 * t153;
        let t392 = 1.0 / t10 / sigma0;
        let t393 = param_beta * t392;
        let t396 = param_beta / sigma0;
        let t398 = t20 * param_gamma * t87;
        let t400 = t203 * rho0;
        let t402 = 1.0 / t11 / t400;
        let t406 = -3.0 / 4.0 * param_beta * t402 * t211 - 3.0 / 4.0 * t393 * t25 + 3.0 / 4.0 * t396 * t398;
        let t410 = t392 * t13;
        let t416 = piecewise3(t15, 2.0 * t16 * t290 * t387 - t16 * t76 * t406 - 2.0 * t145 * t384, -t410 * t34 / 12.0 + t410 * t225 / 6.0);
        let tv3rhosigma20 = -t416 * t11;
        v3rhosigma2[ip * 12] += tv3rhosigma20;
        let tv3rhosigma21 = 0.0;
        v3rhosigma2[ip * 12 + 1] += tv3rhosigma21;
        let tv3rhosigma22 = 0.0;
        v3rhosigma2[ip * 12 + 2] += tv3rhosigma22;
        let tv3rhosigma23 = 0.0;
        v3rhosigma2[ip * 12 + 3] += tv3rhosigma23;
        let tv3rhosigma24 = 0.0;
        v3rhosigma2[ip * 12 + 4] += tv3rhosigma24;
        let tv3rhosigma25 = 0.0;
        v3rhosigma2[ip * 12 + 5] += tv3rhosigma25;
        let tv3rhosigma26 = 0.0;
        v3rhosigma2[ip * 12 + 6] += tv3rhosigma26;
        let tv3rhosigma27 = 0.0;
        v3rhosigma2[ip * 12 + 7] += tv3rhosigma27;
        let tv3rhosigma28 = 0.0;
        v3rhosigma2[ip * 12 + 8] += tv3rhosigma28;
        let tv3rhosigma29 = 0.0;
        v3rhosigma2[ip * 12 + 9] += tv3rhosigma29;
        let tv3rhosigma210 = 0.0;
        v3rhosigma2[ip * 12 + 10] += tv3rhosigma210;
        let t418 = t114 * t172;
        let t421 = t172 * t172;
        let t426 = 1.0 / t39 / sigma2;
        let t427 = param_beta * t426;
        let t430 = param_beta / sigma2;
        let t432 = t49 * param_gamma * t125;
        let t434 = t253 * rho1;
        let t436 = 1.0 / t40 / t434;
        let t440 = -3.0 / 4.0 * param_beta * t436 * t260 - 3.0 / 4.0 * t427 * t54 + 3.0 / 4.0 * t430 * t432;
        let t444 = t426 * t42;
        let t450 = piecewise3(t44, -t45 * t115 * t440 + 2.0 * t45 * t341 * t421 - 2.0 * t164 * t418, -t444 * t63 / 12.0 + t444 * t274 / 6.0);
        let tv3rhosigma211 = -t450 * t40;
        v3rhosigma2[ip * 12 + 11] += tv3rhosigma211;
    }
}
