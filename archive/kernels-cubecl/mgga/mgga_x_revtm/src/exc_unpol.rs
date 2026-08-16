//! MGGA_X_REVTM exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_revtm_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t18 * t19;
        let t21 = 1.0 / rho[ip];
        let t22 = sigma[ip] * t21;
        let t23 = 1.0 / tau[ip];
        let t25 = t22 * t23 / 8.0;
        let t26 = t25 < 1.0;
        let t27 = piecewise3::<f64>(t26, t25, 1.0);
        let t28 = t27 * t27;
        let t29 = t28 * t27;
        let t31 = t28 + 3.0 * t29;
        let t32 = 1.0 + t29;
        let t33 = t32 * t32;
        let t34 = 1.0 / t33;
        let t35 = t31 * t34;
        let t36 = M_CBRT6;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3::<f64>(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t36 * t40;
        let t42 = M_CBRT2;
        let t43 = t42 * t42;
        let t44 = sigma[ip] * t43;
        let t45 = rho[ip] * rho[ip];
        let t46 = t19 * t19;
        let t48 = 1.0 / t46 / t45;
        let t49 = t44 * t48;
        let t50 = t41 * t49;
        let t52 = t36 * t36;
        let t54 = 1.0 / t38 / t37;
        let t55 = t52 * t54;
        let t56 = sigma[ip] * sigma[ip];
        let t57 = t56 * t42;
        let t58 = t45 * t45;
        let t59 = t58 * rho[ip];
        let t61 = 1.0 / t19 / t59;
        let t65 = 1.0 + 0.15045488888888888889e0 * t50 + 0.53798980924525896e-2 * t55 * t57 * t61;
        let t66 = f64::powf(t65, 1.0 / 5.0);
        let t69 = tau[ip] * t43;
        let t71 = 1.0 / t46 / rho[ip];
        let t72 = t69 * t71;
        let t81 = 1.0 + 0.63943327777777777778e-1 * t50 - 5.0 / 9.0 * (0.14554132e0 * t72 + 0.256337604e0 * t52 * t39 + 0.11867481666666666667e-1 * t49) * t36 * t40;
        let t82 = t66 * t66;
        let t83 = 1.0 / t82;
        let t86 = 1.0 / t66 + 7.0 / 9.0 * t81 * t83;
        let t88 = 1.0 - t35;
        let t91 = (10.0 / 81.0 + 25.0 / 8748.0 * t50) * t36;
        let t92 = t91 * t40;
        let t96 = t72 - t49 / 8.0;
        let t97 = t96 * t36;
        let t100 = 5.0 / 9.0 * t97 * t40 - 1.0;
        let t101 = t40 * t100;
        let t104 = 1.0 + 0.22222222222222222222e0 * t97 * t101;
        let t105 = f64::sqrt(t104);
        let t106 = 1.0 / t105;
        let t110 = 9.0 / 20.0 * t100 * t106 + t50 / 36.0;
        let t111 = t110 * t110;
        let t113 = t110 * t27;
        let t114 = 1.0 - t27;
        let t117 = 1.0 + 5.0 / 12.0 * t92 * t49 + 292.0 / 405.0 * t111 - 146.0 / 135.0 * t113 * t114;
        let t118 = f64::powf(t117, 1.0 / 10.0);
        let t120 = t88 * t118 + t35 * t86;
        let t124 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t120);
        let tzk0 = 2.0 * t124;
        zk[ip] += tzk0;
    }
}
