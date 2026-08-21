//! MGGA_X_TASK exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_task.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_task_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_task_c: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_h0x: f64,
    param_task_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = t19 + 1.0;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t34 = t29 / t32;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t39 = 1.0 / t38;
        let t42 = t34 * sigma0 * t39 / 24.0;
        let t43 = 0.0 < t42;
        let t44 = piecewise3(t43, t42, 0.0);
        let t45 = pow_1_4(t44);
        let t48 = rmath::exp(-param_task_c / t45);
        let t50 = piecewise3(t43, 1.0 - t48, 0.0);
        let t52 = tau0 * tau0;
        let t53 = t52 * t52;
        let t54 = t53 * t29;
        let t55 = param_task_bnu_0;
        let t56 = param_task_bnu_1;
        let t57 = param_task_bnu_2;
        let t58 = param_task_bnu_3;
        let t59 = param_task_bnu_4;
        let t60 = t55 + t56 + t57 + t58 + t59;
        let t61 = rho0 * tau0;
        let t65 = 1.0 / rho0;
        let t67 = 1.0 / tau0;
        let t69 = 0.0 < (0.9999999999 * t61 - 0.125 * sigma0) * t65 * t67;
        let t71 = 8.0 * t61 - sigma0;
        let t72 = t71 * t65;
        let t75 = piecewise3(t69, t72 * t67 / 8.0, 1e-10);
        let t76 = t75 * t75;
        let t77 = t76 * t76;
        let t78 = t60 * t77;
        let t81 = t56 / 2.0;
        let t82 = 7.0 / 2.0 * t58;
        let t83 = 7.0 * t59;
        let t85 = t4 * M_PI;
        let t86 = (t55 + t81 - t57 - t82 - t83) * t85;
        let t87 = t52 * tau0;
        let t88 = t37 * rho0;
        let t89 = t87 * t88;
        let t90 = t76 * t75;
        let t94 = t29 * t29;
        let t95 = t4 * t4;
        let t96 = t95 * t30;
        let t97 = t94 * t96;
        let t98 = t35 * rho0;
        let t99 = t36 * t98;
        let t100 = t97 * t99;
        let t103 = t55 - 5.0 / 3.0 * t57 + 35.0 / 3.0 * t59;
        let t104 = t52 * t103;
        let t105 = t104 * t76;
        let t108 = t30 * t30;
        let t110 = t108 * (t55 - t81 - t57 + t82 - t83);
        let t111 = t110 * t29;
        let t112 = t35 * t35;
        let t113 = t112 * rho0;
        let t114 = t113 * tau0;
        let t119 = t37 * t112 * t35;
        let t121 = t4 * t108 * M_PI;
        let t122 = t119 * t121;
        let t123 = t55 - t56 + t57 - t58 + t59;
        let t126 = 14580.0 * t111 * t114 * t75 + 27000.0 * t86 * t89 * t90 + 12150.0 * t100 * t105 + 6561.0 * t122 * t123 + 3750.0 * t54 * t78;
        let t127 = t88 * t85;
        let t129 = tau0 * t29;
        let t132 = 5.0 * t129 * t75 + 9.0 * t127;
        let t133 = t132 * t132;
        let t134 = t133 * t133;
        let t135 = 1.0 / t134;
        let t137 = 1.0 - t126 * t135;
        let t138 = param_task_anu_0;
        let t139 = param_task_anu_1;
        let t140 = param_task_anu_2;
        let t142 = t96 * (t138 - t139 + t140);
        let t146 = t29 * t85;
        let t148 = t138 - 3.0 * t140;
        let t151 = 48.0 * t146 * t148 * t38;
        let t153 = t138 + t139 + t140;
        let t154 = sigma0 * t94 * t153;
        let t157 = 576.0 * t142 * t36 * t113 + (t151 + t154) * sigma0;
        let t161 = t29 * sigma0 + 24.0 * t85 * t38;
        let t162 = t161 * t161;
        let t163 = 1.0 / t162;
        let t165 = t157 * t163 - param_task_h0x;
        let t166 = t137 * t165;
        let t167 = rmath::pow(t50, param_task_d);
        let t168 = t166 * t167;
        let t169 = param_task_h0x * t50 + t168;
        let t173 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t169);
        let t174 = rho1 <= dens_threshold;
        let t175 = -t17;
        let t177 = piecewise5(t15, t12, t11, t16, t175 * t8);
        let t178 = t177 + 1.0;
        let t179 = t178 <= zeta_threshold;
        let t180 = pow_1_3(t178);
        let t182 = piecewise3(t179, t23, t180 * t178);
        let t183 = t182 * t27;
        let t184 = rho1 * rho1;
        let t185 = pow_1_3(rho1);
        let t186 = t185 * t185;
        let t187 = t186 * t184;
        let t188 = 1.0 / t187;
        let t191 = t34 * sigma2 * t188 / 24.0;
        let t192 = 0.0 < t191;
        let t193 = piecewise3(t192, t191, 0.0);
        let t194 = pow_1_4(t193);
        let t197 = rmath::exp(-param_task_c / t194);
        let t199 = piecewise3(t192, 1.0 - t197, 0.0);
        let t201 = tau1 * tau1;
        let t202 = t201 * t201;
        let t203 = t202 * t29;
        let t204 = rho1 * tau1;
        let t208 = 1.0 / rho1;
        let t210 = 1.0 / tau1;
        let t212 = 0.0 < (0.9999999999 * t204 - 0.125 * sigma2) * t208 * t210;
        let t214 = 8.0 * t204 - sigma2;
        let t215 = t214 * t208;
        let t218 = piecewise3(t212, t215 * t210 / 8.0, 1e-10);
        let t219 = t218 * t218;
        let t220 = t219 * t219;
        let t221 = t60 * t220;
        let t224 = t201 * tau1;
        let t225 = t186 * rho1;
        let t226 = t224 * t225;
        let t227 = t219 * t218;
        let t231 = t184 * rho1;
        let t232 = t185 * t231;
        let t233 = t97 * t232;
        let t234 = t201 * t103;
        let t235 = t234 * t219;
        let t238 = t184 * t184;
        let t239 = t238 * rho1;
        let t240 = t239 * tau1;
        let t245 = t186 * t238 * t184;
        let t246 = t245 * t121;
        let t249 = 14580.0 * t111 * t240 * t218 + 27000.0 * t86 * t226 * t227 + 6561.0 * t246 * t123 + 3750.0 * t203 * t221 + 12150.0 * t233 * t235;
        let t250 = t225 * t85;
        let t252 = tau1 * t29;
        let t255 = 5.0 * t252 * t218 + 9.0 * t250;
        let t256 = t255 * t255;
        let t257 = t256 * t256;
        let t258 = 1.0 / t257;
        let t260 = 1.0 - t249 * t258;
        let t266 = 48.0 * t146 * t148 * t187;
        let t268 = sigma2 * t94 * t153;
        let t271 = 576.0 * t142 * t185 * t239 + (t266 + t268) * sigma2;
        let t275 = 24.0 * t85 * t187 + t29 * sigma2;
        let t276 = t275 * t275;
        let t277 = 1.0 / t276;
        let t279 = t271 * t277 - param_task_h0x;
        let t280 = t260 * t279;
        let t281 = rmath::pow(t199, param_task_d);
        let t282 = t280 * t281;
        let t283 = param_task_h0x * t199 + t282;
        let t287 = piecewise3(t174, 0.0, -3.0 / 8.0 * t6 * t183 * t283);
        let tzk0 = t173 + t287;
        zk[ip] += tzk0;
    }
}
