//! MGGA_X_BR89 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_br89_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_at: f64,
    param_gamma: f64,
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
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5::<f64>(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3::<f64>(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3::<f64>(t16);
        let t22 = piecewise3::<f64>(t17, t19, t20 * t16);
        let t23 = pow_1_3::<f64>(t3);
        let t24 = t22 * t23;
        let t26 = pow_1_3::<f64>(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = t24 * t29;
        let t31 = pow_1_3::<f64>(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / rho0;
        let t35 = lapl0 * t34;
        let t37 = tau0 * param_gamma;
        let t38 = t37 * t34;
        let t40 = param_gamma * sigma0;
        let t41 = rho0 * rho0;
        let t43 = 1.0 / t32 / t41;
        let t44 = t40 * t43;
        let t47 = f64::abs(t35 / 2.0 - 2.0 * t38 + t44 / 4.0);
        let t49 = t47 / 3.0 < 0.5e-12;
        let t53 = t35 / 6.0 - 2.0 / 3.0 * t38 + t44 / 12.0;
        let t54 = 0.0 < t53;
        let t55 = piecewise3::<f64>(t54, 0.5e-12, -0.5e-12);
        let t56 = piecewise3::<f64>(t49, t55, t53);
        let t57 = xc_mgga_x_br89_get_x::<f64>(t56);
        let t59 = f64::exp(t57 / 3.0);
        let t60 = f64::exp(-t57);
        let t62 = 1.0 + t57 / 2.0;
        let t63 = t60 * t62;
        let t64 = 1.0 - t63;
        let t65 = t59 * t64;
        let t66 = 1.0 / t57;
        let t67 = M_CBRT6;
        let t68 = t67 * t67;
        let t69 = M_PI * M_PI;
        let t70 = pow_1_3::<f64>(t69);
        let t71 = t70 * t70;
        let t73 = 3.0 / 10.0 * t68 * t71;
        let t74 = tau0 * t34;
        let t75 = t73 - t74;
        let t76 = t73 + t74;
        let t77 = 1.0 / t76;
        let t79 = t75 * t75;
        let t80 = t79 * t75;
        let t81 = t76 * t76;
        let t82 = t81 * t76;
        let t83 = 1.0 / t82;
        let t86 = t79 * t79;
        let t87 = t86 * t75;
        let t88 = t81 * t81;
        let t90 = 1.0 / t88 / t76;
        let t94 = 1.0 + param_at * (t75 * t77 - 2.0 * t80 * t83 + t87 * t90);
        let t95 = t66 * t94;
        let t96 = t65 * t95;
        let t99 = piecewise3::<f64>(t2, 0.0, -t30 * t96 / 4.0);
        let t100 = rho1 <= dens_threshold;
        let t101 = -t13;
        let t103 = piecewise5::<f64>(t11, t8, t7, t12, t101 * t4);
        let t104 = 1.0 + t103;
        let t105 = t104 <= zeta_threshold;
        let t106 = pow_1_3::<f64>(t104);
        let t108 = piecewise3::<f64>(t105, t19, t106 * t104);
        let t109 = t108 * t23;
        let t110 = t109 * t29;
        let t111 = pow_1_3::<f64>(rho1);
        let t112 = t111 * t111;
        let t114 = 1.0 / t112 / rho1;
        let t115 = lapl1 * t114;
        let t117 = tau1 * param_gamma;
        let t118 = t117 * t114;
        let t120 = param_gamma * sigma2;
        let t121 = rho1 * rho1;
        let t123 = 1.0 / t112 / t121;
        let t124 = t120 * t123;
        let t127 = f64::abs(t115 / 2.0 - 2.0 * t118 + t124 / 4.0);
        let t129 = t127 / 3.0 < 0.5e-12;
        let t133 = t115 / 6.0 - 2.0 / 3.0 * t118 + t124 / 12.0;
        let t134 = 0.0 < t133;
        let t135 = piecewise3::<f64>(t134, 0.5e-12, -0.5e-12);
        let t136 = piecewise3::<f64>(t129, t135, t133);
        let t137 = xc_mgga_x_br89_get_x::<f64>(t136);
        let t139 = f64::exp(t137 / 3.0);
        let t140 = f64::exp(-t137);
        let t142 = 1.0 + t137 / 2.0;
        let t143 = t140 * t142;
        let t144 = 1.0 - t143;
        let t145 = t139 * t144;
        let t146 = 1.0 / t137;
        let t147 = tau1 * t114;
        let t148 = t73 - t147;
        let t149 = t73 + t147;
        let t150 = 1.0 / t149;
        let t152 = t148 * t148;
        let t153 = t152 * t148;
        let t154 = t149 * t149;
        let t155 = t154 * t149;
        let t156 = 1.0 / t155;
        let t159 = t152 * t152;
        let t160 = t159 * t148;
        let t161 = t154 * t154;
        let t163 = 1.0 / t161 / t149;
        let t167 = 1.0 + param_at * (t148 * t150 - 2.0 * t153 * t156 + t160 * t163);
        let t168 = t146 * t167;
        let t169 = t145 * t168;
        let t172 = piecewise3::<f64>(t100, 0.0, -t110 * t169 / 4.0);
        let tzk0 = t99 + t172;
        zk[ip] += tzk0;
    }
}
