//! GGA_X_SFAT_PBE exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sfat_pbe_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t18 = t17 / t4 * t3;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t27 = M_CBRT6;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t30 = t29 * t29;
        let t31 = 1.0 / t30;
        let t32 = t31 * t27;
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = t34 * sigma[ip];
        let t36 = rho[ip] * rho[ip];
        let t37 = t19 * t19;
        let t39 = 1.0 / t37 / t36;
        let t43 = 0.804 + 0.009146457198521547 * t39 * t35 * t32;
        let t46 = 1.804 - 0.646416 / t43;
        let t49 = 1.0 / t46 * t25 * t24 * t20 * M_PI;
        let t50 = f64::sqrt(t49);
        let t52 = 1.0 / t50 * param_hyb_omega_0;
        let t53 = rho[ip] * t11;
        let t54 = pow_1_3(t53);
        let t55 = 1.0 / t54;
        let t58 = t55 * t33 * t52 / 2.0;
        let t59 = 1.92 <= t58;
        let t60 = 1.92 < t58;
        let t61 = piecewise3(t60, t58, 1.92);
        let t62 = t61 * t61;
        let t63 = t62 * t62;
        let t64 = 1.0 / t63;
        let t66 = t63 * t62;
        let t67 = 1.0 / t66;
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = t69 * t62;
        let t73 = 1.0 / t72;
        let t75 = t69 * t63;
        let t76 = 1.0 / t75;
        let t78 = t69 * t66;
        let t79 = 1.0 / t78;
        let t81 = t69 * t69;
        let t82 = 1.0 / t81;
        let t85 = 1.0 / t81 / t62;
        let t88 = 1.0 / t81 / t63;
        let t91 = 1.0 / t81 / t66;
        let t94 = 1.0 / t81 / t69;
        let t97 = 1.0 / t81 / t72;
        let t100 = 1.0 / t81 / t75;
        let t103 = 1.0 / t81 / t78;
        let t105 = t81 * t81;
        let t106 = 1.0 / t105;
        let t109 = 1.0 / t105 / t62;
        let t112 = 1.0 / t105 / t63;
        let t116 = -t64 / 30.0 + t67 / 70.0 - t70 / 135.0 + t73 / 231.0 - t76 / 364.0 + t79 / 540.0 - t82 / 765.0 + t85 / 1045.0 - t88 / 1386.0 + t91 / 1794.0 - t94 / 2275.0 + t97 / 2835.0 - t100 / 3480.0 + t103 / 4216.0 - t106 / 5049.0 + t109 / 5985.0 - t112 / 7030.0 + 1.0 / t62 / 9.0;
        let t117 = piecewise3(t60, 1.92, t58);
        let t118 = f64::atan2(1.0, t117);
        let t119 = t117 * t117;
        let t120 = t119 + 3.0;
        let t121 = 1.0 / t119;
        let t122 = 1.0 + t121;
        let t123 = f64::ln(t122);
        let t125 = -t123 * t120 + 1.0;
        let t128 = t118 + t125 * t117 / 4.0;
        let t132 = piecewise3(t59, t116, 1.0 - 8.0 / 3.0 * t128 * t117);
        let t137 = piecewise3(t2, 0.0, -3.0 / 8.0 * t46 * t132 * t19 * t18);
        let tzk0 = 2.0 * t137;
        zk[ip] += tzk0;
    }
}
