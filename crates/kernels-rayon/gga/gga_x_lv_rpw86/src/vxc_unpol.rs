//! GGA_X_LV_RPW86 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lv_rpw86_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 + 0.003931018518518519 * t34;
        let t37 = sigma[ip] * sigma[ip];
        let t38 = t37 * sigma[ip];
        let t39 = t29 * t29;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t42 = t38 * t41;
        let t43 = 3.881824540052514e-07 * t42;
        let t44 = 1.0 + t43;
        let t45 = 1.0 / t44;
        let t48 = t20 * t20;
        let t51 = t48 / t22 / t21;
        let t52 = t37 * t26;
        let t53 = t39 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t60 = 1.0 + 0.077125 * t34 + 0.06017361111111111 * t51 * t52 * t55 + 2.905130394988796e-06 * t42;
        let t61 = rmath::pow(t60, 1.0 / 15.0);
        let t62 = 1.15 + t43;
        let t63 = 1.0 / t62;
        let t64 = t61 * t63;
        let t67 = t36 * t45 + 3.881824540052514e-07 * t42 * t64;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
        let t73 = t17 / t30;
        let t77 = t25 * sigma[ip];
        let t78 = t29 * rho[ip];
        let t80 = 1.0 / t30 / t78;
        let t81 = t27 * t80;
        let t82 = t81 * t45;
        let t85 = t44 * t44;
        let t86 = 1.0 / t85;
        let t87 = t36 * t86;
        let t88 = t40 * rho[ip];
        let t89 = 1.0 / t88;
        let t90 = t38 * t89;
        let t95 = t61 * t61;
        let t96 = t95 * t95;
        let t98 = t96 * t96;
        let t99 = t98 * t96 * t95;
        let t100 = 1.0 / t99;
        let t101 = t100 * t63;
        let t105 = t39 * t29;
        let t107 = 1.0 / t18 / t105;
        let t112 = -0.20566666666666666 * t25 * t28 * t80 - 0.32092592592592595 * t51 * t52 * t107 - 2.324104315991037e-05 * t90;
        let t113 = t101 * t112;
        let t116 = t37 * t37;
        let t117 = t116 * t37;
        let t118 = t40 * t40;
        let t120 = 1.0 / t118 / rho[ip];
        let t121 = t117 * t120;
        let t122 = t62 * t62;
        let t123 = 1.0 / t122;
        let t124 = t61 * t123;
        let t127 = -0.010482716049382716 * t77 * t82 + 3.1054596320420114e-06 * t87 * t90 - 3.1054596320420114e-06 * t90 * t64 + 2.5878830267016762e-08 * t42 * t113 + 1.205484940780313e-12 * t121 * t124;
        let t132 = piecewise3(t2, 0.0, -t6 * t73 * t67 / 8.0 - 3.0 / 8.0 * t6 * t19 * t127);
        let tvrho0 = 2.0 * rho[ip] * t132 + 2.0 * t71;
        vrho[ip] += tvrho0;
        let t135 = t27 * t32;
        let t139 = t37 * t41;
        let t146 = sigma[ip] * t26;
        let t151 = 0.077125 * t25 * t135 + 0.12034722222222222 * t51 * t146 * t55 + 8.715391184966388e-06 * t139;
        let t152 = t101 * t151;
        let t155 = t116 * sigma[ip];
        let t156 = 1.0 / t118;
        let t157 = t155 * t156;
        let t160 = 0.003931018518518519 * t25 * t135 * t45 - 1.1645473620157543e-06 * t87 * t139 + 1.1645473620157543e-06 * t139 * t64 + 2.5878830267016762e-08 * t42 * t152 - 4.5205685279261743e-13 * t157 * t124;
        let t164 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t160);
        let tvsigma0 = 2.0 * rho[ip] * t164;
        vsigma[ip] += tvsigma0;
    }
}
