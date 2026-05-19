//! GGA_X_AM05 exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_am05.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_am05_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_c: f64,
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_alpha * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = t33 * sigma0;
        let t42 = t41 * t39;
        let t43 = t29 * t42;
        let t45 = 1.0 + t43 / 24.0;
        let t46 = 1.0 / t45;
        let t50 = t29 * t41;
        let t51 = t39 * t46;
        let t52 = param_c * t28;
        let t55 = 1.0 + t52 * t42 / 24.0;
        let t56 = t28 * t28;
        let t57 = param_c * t56;
        let t58 = 1.0 / t31;
        let t59 = f64::sqrt(sigma0);
        let t60 = t58 * t59;
        let t62 = 1.0 / t36 / rho0;
        let t65 = 1.0 / M_PI;
        let t66 = t2 * t2;
        let t67 = t65 * t66;
        let t68 = M_CBRT2;
        let t69 = f64::sqrt(12.0);
        let t70 = t56 * t58;
        let t72 = t70 * t59 * t62;
        let t73 = f64::sqrt(t72);
        let t76 = f64::sqrt(6.0);
        let t79 = lambert_w::<f64>(t69 * t73 * t72 * t76 / 1728.0);
        let t80 = pow_1_3::<f64>(t79);
        let t81 = t80 * t80;
        let t83 = t68 * t68;
        let t84 = t2 * t83;
        let t88 = 0.2823705740248932030511071641312341561894e2 + 3.0 / 4.0 * t84 * t80 * t79;
        let t89 = pow_1_4::<f64>(t88);
        let t91 = t67 * t68 * t81 * t89;
        let t94 = 1.0 + t57 * t60 * t62 * t91 / 8.0;
        let t95 = 1.0 / t94;
        let t96 = t55 * t95;
        let t97 = t51 * t96;
        let t100 = 1.0 - t34 * sigma0 * t39 * t46 / 24.0 + t50 * t97 / 24.0;
        let t104 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t100);
        let t105 = rho1 <= dens_threshold;
        let t106 = -t16;
        let t108 = piecewise5::<f64>(t14, t11, t10, t15, t106 * t7);
        let t109 = 1.0 + t108;
        let t110 = t109 <= zeta_threshold;
        let t111 = pow_1_3::<f64>(t109);
        let t113 = piecewise3::<f64>(t110, t22, t111 * t109);
        let t114 = t113 * t26;
        let t115 = rho1 * rho1;
        let t116 = pow_1_3::<f64>(rho1);
        let t117 = t116 * t116;
        let t119 = 1.0 / t117 / t115;
        let t121 = t33 * sigma2;
        let t122 = t121 * t119;
        let t123 = t29 * t122;
        let t125 = 1.0 + t123 / 24.0;
        let t126 = 1.0 / t125;
        let t130 = t29 * t121;
        let t131 = t119 * t126;
        let t134 = 1.0 + t52 * t122 / 24.0;
        let t135 = f64::sqrt(sigma2);
        let t136 = t58 * t135;
        let t138 = 1.0 / t116 / rho1;
        let t142 = t70 * t135 * t138;
        let t143 = f64::sqrt(t142);
        let t148 = lambert_w::<f64>(t69 * t143 * t142 * t76 / 1728.0);
        let t149 = pow_1_3::<f64>(t148);
        let t150 = t149 * t149;
        let t155 = 0.2823705740248932030511071641312341561894e2 + 3.0 / 4.0 * t84 * t149 * t148;
        let t156 = pow_1_4::<f64>(t155);
        let t158 = t67 * t68 * t150 * t156;
        let t161 = 1.0 + t57 * t136 * t138 * t158 / 8.0;
        let t162 = 1.0 / t161;
        let t163 = t134 * t162;
        let t164 = t131 * t163;
        let t167 = 1.0 - t34 * sigma2 * t119 * t126 / 24.0 + t130 * t164 / 24.0;
        let t171 = piecewise3::<f64>(t105, 0.0, -3.0 / 8.0 * t5 * t114 * t167);
        let tzk0 = t104 + t171;
        zk[ip] += tzk0;
    }
}
