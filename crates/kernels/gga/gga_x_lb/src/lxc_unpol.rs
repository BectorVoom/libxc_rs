//! GGA_X_LB lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_vxc/gga_x_lb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_lb_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t1 = M_CBRT3;
        let t4 = pow_1_3::<f64>(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t10 = f64::sqrt(sigma[ip]);
        let t11 = M_CBRT2;
        let t12 = t10 * t11;
        let t13 = pow_1_3::<f64>(rho[ip]);
        let t15 = 1.0 / t13 / rho[ip];
        let t17 = t12 * t15 < 300.0;
        let t18 = param_beta * sigma[ip];
        let t19 = t11 * t11;
        let t20 = rho[ip] * rho[ip];
        let t21 = t13 * t13;
        let t23 = 1.0 / t21 / t20;
        let t24 = t19 * t23;
        let t25 = param_beta * t10;
        let t26 = t11 * t15;
        let t28 = param_gamma * t10 * t26;
        let t29 = f64::ln(t28 + f64::sqrt(t28 * t28 + 1.0));
        let t30 = t26 * t29;
        let t33 = 3.0 * t25 * t30 + 1.0;
        let t34 = 1.0 / t33;
        let t38 = f64::ln(2.0 * t28);
        let t39 = 1.0 / t38;
        let t40 = t15 * t39;
        let t43 = piecewise3::<f64>(t17, t18 * t24 * t34, t12 * t40 / 3.0);
        let t45 = (-param_alpha * t1 * t4 * t6 / 2.0 - t43) * t19;
        let tvrho0 = t45 * t13 / 2.0;
        vrho[ip] += tvrho0;
        let t47 = t20 * rho[ip];
        let t49 = 1.0 / t21 / t47;
        let t54 = t18 * t19;
        let t55 = t33 * t33;
        let t56 = 1.0 / t55;
        let t57 = t23 * t56;
        let t59 = 1.0 / t13 / t20;
        let t61 = t11 * t59 * t29;
        let t64 = param_gamma * param_gamma;
        let t67 = t64 * sigma[ip] * t24 + 1.0;
        let t68 = f64::sqrt(t67);
        let t69 = 1.0 / t68;
        let t70 = t49 * param_gamma * t69;
        let t73 = -4.0 * t25 * t61 - 4.0 * t54 * t70;
        let t74 = t57 * t73;
        let t77 = t59 * t39;
        let t79 = t38 * t38;
        let t80 = 1.0 / t79;
        let t81 = t59 * t80;
        let t85 = piecewise3::<f64>(t17, -8.0 / 3.0 * t18 * t19 * t49 * t34 - t54 * t74, -4.0 / 9.0 * t12 * t77 + 4.0 / 9.0 * t12 * t81);
        let t86 = t85 * t19;
        let t89 = 1.0 / t21;
        let tv2rho20 = -t86 * t13 / 2.0 + t45 * t89 / 6.0;
        v2rho2[ip] += tv2rho20;
        let t92 = param_beta * t19;
        let t95 = 1.0 / t10;
        let t96 = param_beta * t95;
        let t99 = t23 * param_gamma * t69;
        let t102 = 3.0 / 2.0 * t96 * t30 + 3.0 / 2.0 * t92 * t99;
        let t103 = t57 * t102;
        let t106 = t95 * t11;
        let t108 = t15 * t80;
        let t112 = piecewise3::<f64>(t17, t92 * t23 * t34 - t54 * t103, -t106 * t108 / 6.0 + t106 * t40 / 6.0);
        let t113 = t112 * t19;
        let tv2rhosigma0 = -t113 * t13 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t116 = t20 * t20;
        let t118 = 1.0 / t21 / t116;
        let t123 = t49 * t56;
        let t124 = t123 * t73;
        let t128 = 1.0 / t55 / t33;
        let t129 = t23 * t128;
        let t130 = t73 * t73;
        let t131 = t129 * t130;
        let t135 = 1.0 / t13 / t47;
        let t137 = t11 * t135 * t29;
        let t141 = t118 * param_gamma * t69;
        let t144 = sigma[ip] * sigma[ip];
        let t146 = param_beta * t144 * t11;
        let t149 = 1.0 / t13 / t116 / t47;
        let t150 = t64 * param_gamma;
        let t153 = 1.0 / t68 / t67;
        let t157 = 28.0 / 3.0 * t25 * t137 + 20.0 * t54 * t141 - 32.0 / 3.0 * t146 * t149 * t150 * t153;
        let t158 = t57 * t157;
        let t161 = t135 * t39;
        let t164 = t135 * t80;
        let t168 = 1.0 / t79 / t38;
        let t169 = t135 * t168;
        let t173 = piecewise3::<f64>(t17, 88.0 / 9.0 * t18 * t19 * t118 * t34 + 16.0 / 3.0 * t54 * t124 + 2.0 * t54 * t131 - t54 * t158, 28.0 / 27.0 * t12 * t161 - 44.0 / 27.0 * t12 * t164 + 32.0 / 27.0 * t12 * t169);
        let t174 = t173 * t19;
        let t180 = 1.0 / t21 / rho[ip];
        let tv3rho30 = -t174 * t13 / 2.0 - t86 * t89 / 3.0 - t45 * t180 / 9.0;
        v3rho3[ip] += tv3rho30;
        let t187 = t123 * t102;
        let t190 = t102 * t73;
        let t198 = param_beta * t11;
        let t201 = 1.0 / t13 / t116 / t20;
        let t204 = t150 * t153 * sigma[ip];
        let t207 = 4.0 * t198 * t201 * t204 - 2.0 * t96 * t61 - 6.0 * t92 * t70;
        let t208 = t57 * t207;
        let t215 = t59 * t168;
        let t219 = piecewise3::<f64>(t17, -8.0 / 3.0 * t92 * t49 * t34 - t92 * t74 + 8.0 / 3.0 * t54 * t187 + 2.0 * t54 * t129 * t190 - t54 * t208, -2.0 / 9.0 * t106 * t77 + 4.0 / 9.0 * t106 * t81 - 4.0 / 9.0 * t106 * t215);
        let t220 = t219 * t19;
        let tv3rho2sigma0 = -t220 * t13 / 2.0 - t113 * t89 / 6.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t227 = t102 * t102;
        let t228 = t129 * t227;
        let t232 = 1.0 / t10 / sigma[ip];
        let t233 = param_beta * t232;
        let t237 = param_beta / sigma[ip];
        let t238 = t237 * t19;
        let t241 = t116 * rho[ip];
        let t245 = 1.0 / t13 / t241 * t150 * t153;
        let t248 = -3.0 / 4.0 * t233 * t30 + 3.0 / 4.0 * t238 * t99 - 3.0 / 2.0 * t198 * t245;
        let t249 = t57 * t248;
        let t252 = t232 * t11;
        let t255 = t15 * t168;
        let t259 = piecewise3::<f64>(t17, -2.0 * t92 * t103 + 2.0 * t54 * t228 - t54 * t249, -t252 * t40 / 12.0 + t252 * t255 / 6.0);
        let t260 = t259 * t19;
        let tv3rhosigma20 = -t260 * t13 / 2.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t264 = 1.0 / t21 / t241;
        let t269 = t118 * t56;
        let t273 = t49 * t128;
        let t280 = t55 * t55;
        let t282 = t23 / t280;
        let t292 = 1.0 / t13 / t116;
        let t301 = t116 * t116;
        let t312 = t64 * t64;
        let t313 = t312 * param_gamma;
        let t315 = t67 * t67;
        let t317 = 1.0 / t68 / t315;
        let t334 = t79 * t79;
        let t335 = 1.0 / t334;
        let t340 = piecewise3::<f64>(t17, -1232.0 / 27.0 * t18 * t19 * t264 * t34 - 88.0 / 3.0 * t54 * t269 * t73 - 16.0 * t54 * t273 * t130 + 8.0 * t54 * t123 * t157 - 6.0 * t54 * t282 * t130 * t73 + 6.0 * t54 * t129 * t73 * t157 - t54 * t57 * (-280.0 / 9.0 * t25 * t11 * t292 * t29 - 952.0 / 9.0 * t54 * t264 * param_gamma * t69 + 1184.0 / 9.0 * t146 / t13 / t301 * t150 * t153 - 256.0 / 3.0 * param_beta * t144 * sigma[ip] / t301 / t47 * t313 * t317), -280.0 / 81.0 * t12 * t292 * t39 + 184.0 / 27.0 * t12 * t292 * t80 - 224.0 / 27.0 * t12 * t292 * t168 + 128.0 / 27.0 * t12 * t292 * t335);
        let tv4rho40 = -t340 * t19 * t13 / 2.0 - t174 * t89 / 2.0 + t86 * t180 / 3.0 + 5.0 / 27.0 * t45 * t23;
        v4rho4[ip] += tv4rho40;
        let t389 = t313 * t317;
        let t396 = 88.0 / 9.0 * t92 * t118 * t34 + 16.0 / 3.0 * t92 * t124 + 2.0 * t92 * t131 - t92 * t158 - 88.0 / 9.0 * t54 * t269 * t102 - 32.0 / 3.0 * t54 * t273 * t190 + 16.0 / 3.0 * t54 * t123 * t207 - 6.0 * t54 * t282 * t102 * t130 + 4.0 * t54 * t129 * t207 * t73 + 2.0 * t54 * t129 * t102 * t157 - t54 * t57 * (14.0 / 3.0 * t96 * t137 + 74.0 / 3.0 * t92 * t141 - 124.0 / 3.0 * t198 * t149 * t204 + 32.0 * param_beta / t301 / t20 * t389 * t144);
        let t407 = piecewise3::<f64>(t17, t396, 14.0 / 27.0 * t106 * t161 - 4.0 / 3.0 * t106 * t164 + 20.0 / 9.0 * t106 * t169 - 16.0 / 9.0 * t106 * t135 * t335);
        let tv4rho3sigma0 = -t407 * t19 * t13 / 2.0 - t220 * t89 / 3.0 + t113 * t180 / 9.0;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t468 = piecewise3::<f64>(t17, 16.0 / 3.0 * t92 * t187 + 4.0 * t92 * t23 * t128 * t102 * t73 - 2.0 * t92 * t208 - 16.0 / 3.0 * t54 * t273 * t227 - 6.0 * t54 * t282 * t227 * t73 + 4.0 * t54 * t129 * t102 * t207 + 8.0 / 3.0 * t54 * t123 * t248 + 2.0 * t54 * t129 * t248 * t73 - t54 * t57 * (t233 * t61 - t238 * t70 + 10.0 * t198 * t201 * t150 * t153 - 12.0 * param_beta / t301 / rho[ip] * t389 * sigma[ip]), t252 * t77 / 9.0 - t252 * t81 / 9.0 - 2.0 / 9.0 * t252 * t215 + 2.0 / 3.0 * t252 * t59 * t335);
        let tv4rho2sigma20 = -t468 * t19 * t13 / 2.0 - t260 * t89 / 6.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t487 = 1.0 / t10 / t144;
        let t507 = t487 * t11;
        let t518 = piecewise3::<f64>(t17, 6.0 * t92 * t228 - 3.0 * t92 * t249 - 6.0 * t54 * t282 * t227 * t102 + 6.0 * t54 * t129 * t102 * t248 - t54 * t57 * (9.0 / 8.0 * param_beta * t487 * t30 - 9.0 / 8.0 * param_beta / t144 * t19 * t99 - 3.0 / 4.0 * t237 * t11 * t245 + 9.0 / 2.0 * param_beta / t301 * t389), t507 * t40 / 8.0 + t507 * t108 / 24.0 - t507 * t255 / 4.0 - t507 * t15 * t335 / 4.0);
        let tv4rhosigma30 = -t518 * t19 * t13 / 2.0;
        v4rhosigma3[ip] += tv4rhosigma30;
    }
}
