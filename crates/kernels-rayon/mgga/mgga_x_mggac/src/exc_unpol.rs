//! MGGA_X_MGGAC exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mggac.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mggac_exc_unpol(
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
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(32.0);
        let t22 = t21 * t21;
        let t23 = t5 * t5;
        let t24 = t22 * t23;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t19 * t19;
        let t30 = 1.0 / t28 / rho[ip];
        let t31 = t27 * t30;
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = 1.0 / rho[ip];
        let t39 = sigma[ip] * t38;
        let t40 = 1.0 / tau[ip];
        let t42 = t39 * t40 / 8.0;
        let t44 = 0.0 < 0.9999999999 - t42;
        let t46 = piecewise3(t44, 1.0 - t42, 1e-10);
        let t47 = t37 * t46;
        let t48 = t31 * t47;
        let t50 = tau[ip] * tau[ip];
        let t51 = t50 * t25;
        let t52 = rho[ip] * rho[ip];
        let t53 = t52 * rho[ip];
        let t55 = 1.0 / t19 / t53;
        let t56 = t51 * t55;
        let t57 = t32 * t32;
        let t59 = 1.0 / t34 / t33;
        let t60 = t57 * t59;
        let t61 = t46 * t46;
        let t62 = t60 * t61;
        let t63 = t56 * t62;
        let t65 = 1.0 + 0.05555555555555555 * t48 - 6.972166666666666 * t63;
        let t68 = 3.712 + 1.1111111111111112 * t48 + 2.3240555555555558 * t63;
        let t69 = 1.0 / t68;
        let t73 = xc_mgga_x_mbrxc_get_x(t24 * t65 * t69 / 6.0);
        let t74 = pow_1_4(f64::EPSILON);
        let t75 = t73 < t74;
        let t76 = t21 * t5;
        let t77 = t4 * t4;
        let t78 = 1.0 / M_PI;
        let t79 = pow_1_3(t78);
        let t80 = 1.0 / t79;
        let t81 = t77 * t80;
        let t82 = M_CBRT4;
        let t84 = t76 * t81 * t82;
        let t86 = t76 * t77;
        let t87 = t80 * t82;
        let t88 = t73 * t73;
        let t89 = t87 * t88;
        let t92 = t88 * t73;
        let t93 = t87 * t92;
        let t96 = t88 * t88;
        let t97 = t87 * t96;
        let t100 = t96 * t73;
        let t101 = t87 * t100;
        let t104 = t96 * t88;
        let t105 = t87 * t104;
        let t113 = t76 * t81;
        let t114 = t74 < t73;
        let t115 = piecewise3(t114, t73, t74);
        let t117 = f64::exp(t115 / 3.0);
        let t118 = t82 * t117;
        let t119 = f64::exp(-t115);
        let t120 = t115 * t115;
        let t122 = t120 + 5.0 * t115 + 8.0;
        let t123 = t119 * t122;
        let t124 = 8.0 - t123;
        let t125 = 1.0 / t115;
        let t126 = t124 * t125;
        let t127 = 1.0 + t115;
        let t128 = pow_1_3(t127);
        let t129 = 1.0 / t128;
        let t130 = t126 * t129;
        let t134 = piecewise3(t75, -t84 / 12.0 - t86 * t89 / 108.0 + t86 * t93 / 108.0 - 13.0 / 1620.0 * t86 * t97 + 67.0 / 9720.0 * t86 * t101 - 52.0 / 8505.0 * t86 * t105 + 1811.0 / 326592.0 * t86 * t87 * t96 * t92, -t113 * t118 * t130 / 36.0);
        let t138 = piecewise3(t3, 0.0, 3.0 / 16.0 * t7 * t20 * t134);
        let tzk0 = 2.0 * t138;
        zk[ip] += tzk0;
    }
}
