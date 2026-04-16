//! HYB_GGA_X_CAM_S12 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_x_cam_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_gga_x_cam_s12_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_hyb_coeff_0: f64,
    param_hyb_coeff_1: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = param_C * sigma[ip];
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = t19 * t19;
        let t26 = 1.0 / t24 / t23;
        let t27 = t22 * t26;
        let t29 = sigma[ip] * sigma[ip];
        let t30 = param_D * t29;
        let t31 = t23 * t23;
        let t32 = t31 * rho[ip];
        let t34 = 1.0 / t19 / t32;
        let t35 = t21 * t34;
        let t38 = t20 * t27 + 2.0 * t30 * t35 + 1.0;
        let t41 = param_B * (1.0 - 1.0 / t38);
        let t42 = param_E * sigma[ip];
        let t44 = t42 * t27 + 1.0;
        let t46 = 1.0 - 1.0 / t44;
        let t48 = t41 * t46 + param_A;
        let t49 = t19 * t48;
        let t50 = t3 * t3;
        let t52 = 1.0 / M_PI;
        let t53 = pow_1_3(t52);
        let t54 = 1.0 / t53;
        let t55 = M_CBRT4;
        let t56 = t54 * t55;
        let t59 = M_PI * t50 * t56 / t48;
        let t60 = f64::sqrt(t59);
        let t62 = param_hyb_omega_0 / t60;
        let t63 = t11 * rho[ip];
        let t64 = pow_1_3(t63);
        let t65 = 1.0 / t64;
        let t66 = t21 * t65;
        let t68 = t62 * t66 / 2.0;
        let t69 = 0.135e1 <= t68;
        let t70 = 0.135e1 < t68;
        let t71 = piecewise3(t70, t68, 0.135e1);
        let t72 = t71 * t71;
        let t75 = t72 * t72;
        let t76 = 1.0 / t75;
        let t78 = t75 * t72;
        let t79 = 1.0 / t78;
        let t81 = t75 * t75;
        let t82 = 1.0 / t81;
        let t85 = 1.0 / t81 / t72;
        let t88 = 1.0 / t81 / t75;
        let t91 = 1.0 / t81 / t78;
        let t93 = t81 * t81;
        let t94 = 1.0 / t93;
        let t97 = piecewise3(t70, 0.135e1, t68);
        let t98 = f64::sqrt(M_PI);
        let t99 = 1.0 / t97;
        let t101 = erf_approx(t99 / 2.0);
        let t103 = t97 * t97;
        let t104 = 1.0 / t103;
        let t106 = f64::exp(-t104 / 4.0);
        let t107 = t106 - 1.0;
        let t110 = t106 - 3.0 / 2.0 - 2.0 * t103 * t107;
        let t113 = t98 * t101 + 2.0 * t97 * t110;
        let t117 = piecewise3(t69, 1.0 / t72 / 36.0 - t76 / 960.0 + t79 / 26880.0 - t82 / 829440.0 + t85 / 28385280.0 - t88 / 0.107347968e10 + t91 / 0.445906944e11 - t94 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t97 * t113);
        let t119 = -param_hyb_coeff_0 * t117 - param_hyb_coeff_1 + 1.0;
        let t123 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t49 * t119);
        let tzk0 = 2.0 * t123;
        zk[ip] += tzk0;
        let t124 = 1.0 / t24;
        let t125 = t124 * t48;
        let t129 = t38 * t38;
        let t131 = param_B / t129;
        let t132 = t23 * rho[ip];
        let t134 = 1.0 / t24 / t132;
        let t135 = t22 * t134;
        let t138 = t31 * t23;
        let t140 = 1.0 / t19 / t138;
        let t141 = t21 * t140;
        let t144 = -8.0 / 3.0 * t20 * t135 - 32.0 / 3.0 * t30 * t141;
        let t145 = t144 * t46;
        let t147 = t44 * t44;
        let t148 = 1.0 / t147;
        let t149 = t41 * t148;
        let t150 = t42 * t135;
        let t153 = t131 * t145 - 8.0 / 3.0 * t149 * t150;
        let t154 = t19 * t153;
        let t158 = t72 * t71;
        let t159 = 1.0 / t158;
        let t162 = param_hyb_omega_0 / t60 / t59;
        let t164 = t162 * t66 * M_PI;
        let t165 = t50 * t54;
        let t166 = t48 * t48;
        let t167 = 1.0 / t166;
        let t168 = t55 * t167;
        let t174 = 1.0 / t64 / t63;
        let t175 = t21 * t174;
        let t179 = t164 * t165 * t168 * t153 / 4.0 - t62 * t175 * t11 / 6.0;
        let t180 = piecewise3(t70, t179, 0.0);
        let t183 = t75 * t71;
        let t184 = 1.0 / t183;
        let t187 = t75 * t158;
        let t188 = 1.0 / t187;
        let t192 = 1.0 / t81 / t71;
        let t196 = 1.0 / t81 / t158;
        let t200 = 1.0 / t81 / t183;
        let t204 = 1.0 / t81 / t187;
        let t208 = 1.0 / t93 / t71;
        let t212 = piecewise3(t70, 0.0, t179);
        let t214 = t106 * t104;
        let t218 = t103 * t97;
        let t219 = 1.0 / t218;
        let t223 = t97 * t107;
        let t228 = t219 * t212 * t106 / 2.0 - 4.0 * t223 * t212 - t99 * t212 * t106;
        let t231 = 2.0 * t212 * t110 - t214 * t212 + 2.0 * t97 * t228;
        let t235 = piecewise3(t69, -t159 * t180 / 18.0 + t184 * t180 / 240.0 - t188 * t180 / 4480.0 + t192 * t180 / 103680.0 - t196 * t180 / 2838528.0 + t200 * t180 / 89456640.0 - t204 * t180 / 0.31850496e10 + t208 * t180 / 0.1263403008e12, -8.0 / 3.0 * t212 * t113 - 8.0 / 3.0 * t97 * t231);
        let t236 = param_hyb_coeff_0 * t235;
        let t241 = piecewise3(t2, 0.0, -t18 * t125 * t119 / 8.0 - 3.0 / 8.0 * t18 * t154 * t119 + 3.0 / 8.0 * t18 * t49 * t236);
        let tvrho0 = 2.0 * rho[ip] * t241 + 2.0 * t123;
        vrho[ip] += tvrho0;
        let t244 = param_C * t22;
        let t246 = param_D * sigma[ip];
        let t249 = t244 * t26 + 4.0 * t246 * t35;
        let t250 = t249 * t46;
        let t252 = param_E * t22;
        let t255 = t149 * t252 * t26 + t131 * t250;
        let t256 = t19 * t255;
        let t262 = t164 * t165 * t168 * t255 / 4.0;
        let t263 = piecewise3(t70, t262, 0.0);
        let t266 = t184 * t263;
        let t268 = t188 * t263;
        let t270 = t192 * t263;
        let t272 = t196 * t263;
        let t274 = t200 * t263;
        let t276 = t204 * t263;
        let t278 = t208 * t263;
        let t281 = piecewise3(t70, 0.0, t262);
        let t293 = t219 * t281 * t106 / 2.0 - 4.0 * t223 * t281 - t99 * t281 * t106;
        let t296 = 2.0 * t281 * t110 - t214 * t281 + 2.0 * t97 * t293;
        let t300 = piecewise3(t69, -t159 * t263 / 18.0 + t266 / 240.0 - t268 / 4480.0 + t270 / 103680.0 - t272 / 2838528.0 + t274 / 89456640.0 - t276 / 0.31850496e10 + t278 / 0.1263403008e12, -8.0 / 3.0 * t281 * t113 - 8.0 / 3.0 * t97 * t296);
        let t301 = param_hyb_coeff_0 * t300;
        let t306 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t256 * t119 + 3.0 / 8.0 * t18 * t49 * t301);
        let tvsigma0 = 2.0 * rho[ip] * t306;
        vsigma[ip] += tvsigma0;
    }
}
