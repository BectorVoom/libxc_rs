//! MGGA_X_RPPSCAN exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_rppscan_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t38 = t37 * t35;
        let t39 = 1.0 / t38;
        let t40 = sigma0 * t39;
        let t41 = t34 * t40;
        let t45 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t46 = t29 * t29;
        let t47 = t45 * t46;
        let t48 = t31 * t30;
        let t49 = 1.0 / t48;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t35 * t35;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t36 / t53;
        let t56 = t51 * t55;
        let t57 = t45 * t29;
        let t58 = t33 * sigma0;
        let t59 = t58 * t39;
        let t62 = f64::exp(-27.0 / 80.0 * t57 * t59);
        let t66 = f64::sqrt(146.0);
        let t67 = t66 * t29;
        let t70 = t37 * rho0;
        let t71 = 1.0 / t70;
        let t74 = tau0 * t71 - t40 / 8.0;
        let t76 = 3.0 / 10.0 * t46 * t32;
        let t77 = param_eta * sigma0;
        let t80 = t76 + t77 * t39 / 8.0;
        let t81 = 1.0 / t80;
        let t82 = t74 * t81;
        let t83 = 1.0 - t82;
        let t85 = t83 * t83;
        let t87 = f64::exp(-t85 / 2.0);
        let t90 = 7.0 / 12960.0 * t67 * t59 + t66 * t83 * t87 / 100.0;
        let t91 = t90 * t90;
        let t92 = param_k1 + 5.0 / 972.0 * t41 + t50 * t56 * t62 / 576.0 + t91;
        let t97 = 1.0 + param_k1 * (1.0 - param_k1 / t92);
        let t98 = t82 <= 0.25e1;
        let t99 = 0.25e1 < t82;
        let t100 = piecewise3(t99, 0.25e1, t82);
        let t102 = t100 * t100;
        let t104 = t102 * t100;
        let t106 = t102 * t102;
        let t108 = t106 * t100;
        let t110 = t106 * t102;
        let t115 = piecewise3(t99, t82, 0.25e1);
        let t116 = 1.0 - t115;
        let t119 = f64::exp(param_c2 / t116);
        let t121 = piecewise3(t98, 1.0 - 0.667e0 * t100 - 0.4445555e0 * t102 - 0.663086601049e0 * t104 + 0.145129704449e1 * t106 - 0.887998041597e0 * t108 + 0.234528941479e0 * t110 - 0.23185843322e-1 * t106 * t104, -param_d * t119);
        let t122 = 1.0 - t121;
        let t125 = t97 * t122 + 0.1174e1 * t121;
        let t126 = t28 * t125;
        let t127 = f64::sqrt(3.0);
        let t128 = 1.0 / t31;
        let t129 = t46 * t128;
        let t130 = f64::sqrt(sigma0);
        let t131 = t36 * rho0;
        let t132 = 1.0 / t131;
        let t134 = t129 * t130 * t132;
        let t135 = f64::sqrt(t134);
        let t139 = f64::exp(-0.98958e1 * t127 / t135);
        let t140 = 1.0 - t139;
        let t141 = t126 * t140;
        let t144 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t141);
        let t145 = rho1 <= dens_threshold;
        let t146 = -t17;
        let t148 = piecewise5(t15, t12, t11, t16, t146 * t8);
        let t149 = 1.0 + t148;
        let t150 = t149 <= zeta_threshold;
        let t151 = pow_1_3(t149);
        let t153 = piecewise3(t150, t23, t151 * t149);
        let t154 = t6 * t153;
        let t155 = rho1 * rho1;
        let t156 = pow_1_3(rho1);
        let t157 = t156 * t156;
        let t158 = t157 * t155;
        let t159 = 1.0 / t158;
        let t160 = sigma2 * t159;
        let t161 = t34 * t160;
        let t163 = sigma2 * sigma2;
        let t164 = t155 * t155;
        let t165 = t164 * rho1;
        let t167 = 1.0 / t156 / t165;
        let t168 = t163 * t167;
        let t169 = t33 * sigma2;
        let t170 = t169 * t159;
        let t173 = f64::exp(-27.0 / 80.0 * t57 * t170);
        let t179 = t157 * rho1;
        let t180 = 1.0 / t179;
        let t183 = tau1 * t180 - t160 / 8.0;
        let t184 = param_eta * sigma2;
        let t187 = t76 + t184 * t159 / 8.0;
        let t188 = 1.0 / t187;
        let t189 = t183 * t188;
        let t190 = 1.0 - t189;
        let t192 = t190 * t190;
        let t194 = f64::exp(-t192 / 2.0);
        let t197 = 7.0 / 12960.0 * t67 * t170 + t66 * t190 * t194 / 100.0;
        let t198 = t197 * t197;
        let t199 = param_k1 + 5.0 / 972.0 * t161 + t50 * t168 * t173 / 576.0 + t198;
        let t204 = 1.0 + param_k1 * (1.0 - param_k1 / t199);
        let t205 = t189 <= 0.25e1;
        let t206 = 0.25e1 < t189;
        let t207 = piecewise3(t206, 0.25e1, t189);
        let t209 = t207 * t207;
        let t211 = t209 * t207;
        let t213 = t209 * t209;
        let t215 = t213 * t207;
        let t217 = t213 * t209;
        let t222 = piecewise3(t206, t189, 0.25e1);
        let t223 = 1.0 - t222;
        let t226 = f64::exp(param_c2 / t223);
        let t228 = piecewise3(t205, 1.0 - 0.667e0 * t207 - 0.4445555e0 * t209 - 0.663086601049e0 * t211 + 0.145129704449e1 * t213 - 0.887998041597e0 * t215 + 0.234528941479e0 * t217 - 0.23185843322e-1 * t213 * t211, -param_d * t226);
        let t229 = 1.0 - t228;
        let t232 = t204 * t229 + 0.1174e1 * t228;
        let t233 = t28 * t232;
        let t234 = f64::sqrt(sigma2);
        let t235 = t156 * rho1;
        let t236 = 1.0 / t235;
        let t238 = t129 * t234 * t236;
        let t239 = f64::sqrt(t238);
        let t243 = f64::exp(-0.98958e1 * t127 / t239);
        let t244 = 1.0 - t243;
        let t245 = t233 * t244;
        let t248 = piecewise3(t145, 0.0, -3.0 / 8.0 * t154 * t245);
        let tzk0 = t144 + t248;
        zk[ip] += tzk0;
    }
}
