//! MGGA_X_R2SCAN exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_r2scan_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
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
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3::<f64>(t20);
        let t26 = piecewise3::<f64>(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3::<f64>(t7);
        let t30 = 20.0 / 27.0 + 5.0 / 3.0 * param_eta;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3::<f64>(t33);
        let t35 = t34 * t33;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = sigma0 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = t39 * t39;
        let t41 = t40 * rho0;
        let t42 = pow_1_3::<f64>(rho0);
        let t44 = 1.0 / t42 / t41;
        let t45 = t38 * t44;
        let t46 = param_dp2 * param_dp2;
        let t47 = t46 * t46;
        let t48 = 1.0 / t47;
        let t52 = f64::exp(-t37 * t45 * t48 / 576.0);
        let t56 = (-0.162742215233874e0 * t30 * t52 + 10.0 / 81.0) * t31;
        let t57 = t34 * t34;
        let t58 = 1.0 / t57;
        let t59 = t58 * sigma0;
        let t60 = t42 * t42;
        let t61 = t60 * t39;
        let t62 = 1.0 / t61;
        let t66 = param_k1 + t56 * t59 * t62 / 24.0;
        let t70 = param_k1 * (1.0 - param_k1 / t66);
        let t71 = t60 * rho0;
        let t72 = 1.0 / t71;
        let t74 = sigma0 * t62;
        let t76 = tau0 * t72 - t74 / 8.0;
        let t78 = 3.0 / 10.0 * t32 * t57;
        let t79 = param_eta * sigma0;
        let t82 = t78 + t79 * t62 / 8.0;
        let t83 = 1.0 / t82;
        let t84 = t76 * t83;
        let t85 = t84 <= 0.0;
        let t86 = 0.0 < t84;
        let t87 = piecewise3::<f64>(t86, 0.0, t84);
        let t88 = param_c1 * t87;
        let t89 = 1.0 - t87;
        let t90 = 1.0 / t89;
        let t92 = f64::exp(-t88 * t90);
        let t93 = t84 <= 0.25e1;
        let t94 = 0.25e1 < t84;
        let t95 = piecewise3::<f64>(t94, 0.25e1, t84);
        let t97 = t95 * t95;
        let t99 = t97 * t95;
        let t101 = t97 * t97;
        let t103 = t101 * t95;
        let t105 = t101 * t97;
        let t110 = piecewise3::<f64>(t94, t84, 0.25e1);
        let t111 = 1.0 - t110;
        let t114 = f64::exp(param_c2 / t111);
        let t116 = piecewise5::<f64>(t85, t92, t93, 1.0 - 0.667e0 * t95 - 0.4445555e0 * t97 - 0.663086601049e0 * t99 + 0.145129704449e1 * t101 - 0.887998041597e0 * t103 + 0.234528941479e0 * t105 - 0.23185843322e-1 * t101 * t99, -param_d * t114);
        let t117 = 0.174e0 - t70;
        let t119 = t116 * t117 + t70 + 1.0;
        let t120 = t28 * t119;
        let t121 = f64::sqrt(3.0);
        let t122 = 1.0 / t34;
        let t123 = t32 * t122;
        let t124 = f64::sqrt(sigma0);
        let t125 = t42 * rho0;
        let t126 = 1.0 / t125;
        let t128 = t123 * t124 * t126;
        let t129 = f64::sqrt(t128);
        let t133 = f64::exp(-0.98958e1 * t121 / t129);
        let t134 = 1.0 - t133;
        let t135 = t120 * t134;
        let t138 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t27 * t135);
        let t139 = rho1 <= dens_threshold;
        let t140 = -t17;
        let t142 = piecewise5::<f64>(t15, t12, t11, t16, t140 * t8);
        let t143 = 1.0 + t142;
        let t144 = t143 <= zeta_threshold;
        let t145 = pow_1_3::<f64>(t143);
        let t147 = piecewise3::<f64>(t144, t23, t145 * t143);
        let t148 = t6 * t147;
        let t149 = sigma2 * sigma2;
        let t150 = rho1 * rho1;
        let t151 = t150 * t150;
        let t152 = t151 * rho1;
        let t153 = pow_1_3::<f64>(rho1);
        let t155 = 1.0 / t153 / t152;
        let t156 = t149 * t155;
        let t160 = f64::exp(-t37 * t156 * t48 / 576.0);
        let t164 = (-0.162742215233874e0 * t30 * t160 + 10.0 / 81.0) * t31;
        let t165 = t58 * sigma2;
        let t166 = t153 * t153;
        let t167 = t166 * t150;
        let t168 = 1.0 / t167;
        let t172 = param_k1 + t164 * t165 * t168 / 24.0;
        let t176 = param_k1 * (1.0 - param_k1 / t172);
        let t177 = t166 * rho1;
        let t178 = 1.0 / t177;
        let t180 = sigma2 * t168;
        let t182 = tau1 * t178 - t180 / 8.0;
        let t183 = param_eta * sigma2;
        let t186 = t78 + t183 * t168 / 8.0;
        let t187 = 1.0 / t186;
        let t188 = t182 * t187;
        let t189 = t188 <= 0.0;
        let t190 = 0.0 < t188;
        let t191 = piecewise3::<f64>(t190, 0.0, t188);
        let t192 = param_c1 * t191;
        let t193 = 1.0 - t191;
        let t194 = 1.0 / t193;
        let t196 = f64::exp(-t192 * t194);
        let t197 = t188 <= 0.25e1;
        let t198 = 0.25e1 < t188;
        let t199 = piecewise3::<f64>(t198, 0.25e1, t188);
        let t201 = t199 * t199;
        let t203 = t201 * t199;
        let t205 = t201 * t201;
        let t207 = t205 * t199;
        let t209 = t205 * t201;
        let t214 = piecewise3::<f64>(t198, t188, 0.25e1);
        let t215 = 1.0 - t214;
        let t218 = f64::exp(param_c2 / t215);
        let t220 = piecewise5::<f64>(t189, t196, t197, 1.0 - 0.667e0 * t199 - 0.4445555e0 * t201 - 0.663086601049e0 * t203 + 0.145129704449e1 * t205 - 0.887998041597e0 * t207 + 0.234528941479e0 * t209 - 0.23185843322e-1 * t205 * t203, -param_d * t218);
        let t221 = 0.174e0 - t176;
        let t223 = t220 * t221 + t176 + 1.0;
        let t224 = t28 * t223;
        let t225 = f64::sqrt(sigma2);
        let t226 = t153 * rho1;
        let t227 = 1.0 / t226;
        let t229 = t123 * t225 * t227;
        let t230 = f64::sqrt(t229);
        let t234 = f64::exp(-0.98958e1 * t121 / t230);
        let t235 = 1.0 - t234;
        let t236 = t224 * t235;
        let t239 = piecewise3::<f64>(t139, 0.0, -3.0 / 8.0 * t148 * t236);
        let tzk0 = t138 + t239;
        zk[ip] += tzk0;
    }
}
