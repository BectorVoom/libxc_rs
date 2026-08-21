//! MGGA_X_REGTM exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_regtm_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = tau[ip] * t22;
        let t24 = t19 * t19;
        let t26 = 1.0 / t24 / rho[ip];
        let t27 = t23 * t26;
        let t28 = sigma[ip] * t22;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t24 / t29;
        let t32 = t28 * t31;
        let t34 = t27 - t32 / 8.0;
        let t35 = M_CBRT6;
        let t36 = t34 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t35 * t40;
        let t42 = t41 * t32;
        let t44 = t36 * t40;
        let t46 = 1.0 - 5.0 / 9.0 * t44;
        let t47 = t46 * t46;
        let t48 = t47 * t46;
        let t49 = t34 * t34;
        let t50 = t35 * t35;
        let t51 = t49 * t50;
        let t53 = 1.0 / t38 / t37;
        let t56 = 1.0 + 0.6714891975308642 * t51 * t53;
        let t57 = rmath::sqrt(t56);
        let t59 = 1.0 / t57 / t56;
        let t60 = t48 * t59;
        let t62 = rmath::exp(-t42 / 8.0);
        let t64 = t42 / 24.0 + t60 * t62;
        let t65 = 1.0 / t64;
        let t66 = t40 * t65;
        let t69 = 1.0 + t36 * t66 / 3.0;
        let t70 = t69 * t69;
        let t72 = t70 * t69;
        let t73 = 1.0 / t72;
        let t75 = 1.0 / t70 + 3.0 * t73;
        let t76 = 1.0 + t73;
        let t77 = t76 * t76;
        let t78 = 1.0 / t77;
        let t79 = t75 * t78;
        let t81 = t50 * t53;
        let t82 = sigma[ip] * sigma[ip];
        let t83 = t82 * t21;
        let t84 = t29 * t29;
        let t85 = t84 * rho[ip];
        let t87 = 1.0 / t19 / t85;
        let t91 = 1.0 + 0.1504548888888889 * t42 + 0.00537989809245259 * t81 * t83 * t87;
        let t92 = rmath::pow(t91, 1.0 / 5.0);
        let t103 = 1.0 + 0.06394332777777778 * t42 - 5.0 / 9.0 * (0.14554132 * t27 + 0.256337604 * t50 * t39 + 0.011867481666666667 * t32) * t35 * t40;
        let t104 = t92 * t92;
        let t105 = 1.0 / t104;
        let t108 = 1.0 / t92 + 7.0 / 9.0 * t103 * t105;
        let t110 = 1.0 - t79;
        let t113 = (10.0 / 81.0 + 25.0 / 8748.0 * t42) * t35;
        let t114 = t113 * t40;
        let t119 = t44 / 4.0 - 9.0 / 20.0 + t42 / 36.0;
        let t120 = t119 * t119;
        let t122 = 1.0 / rho[ip];
        let t123 = sigma[ip] * t122;
        let t124 = 1.0 / tau[ip];
        let t126 = t123 * t124 / 8.0;
        let t127 = t126 < 1.0;
        let t128 = piecewise3(t127, t126, 1.0);
        let t129 = t119 * t128;
        let t130 = 1.0 - t128;
        let t133 = 1.0 + 5.0 / 12.0 * t114 * t32 + 292.0 / 405.0 * t120 - 146.0 / 135.0 * t129 * t130;
        let t134 = rmath::pow(t133, 1.0 / 10.0);
        let t136 = t108 * t79 + t110 * t134;
        let t140 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t136);
        let tzk0 = 2.0 * t140;
        zk[ip] += tzk0;
    }
}
