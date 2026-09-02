//! GGA_X_Q1D vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q1d_vxc_unpol(
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
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t25 * t33;
        let t36 = 0.804 + 5.0 / 972.0 * t34;
        let t38 = 0.646416 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t18 / t47;
        let t52 = t43 * t45 * t49 / 288.0;
        let t53 = t34 / 24.0 + t52;
        let t54 = t21 * t21;
        let t55 = 1.0 / t54;
        let t56 = t44 * sigma[ip];
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = 1.0 + t52 + t57 * t59 / 576.0;
        let t63 = 1.0 / t62;
        let t64 = t53 * t63;
        let t66 = (1.804 - t38) * t20;
        let t67 = t66 * t24;
        let t70 = -t67 * t33 / 24.0 + 0.06525;
        let t72 = 1.804 - t38 + t64 * t70;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
        let t78 = t17 / t30;
        let t82 = t36 * t36;
        let t83 = 1.0 / t82;
        let t84 = t83 * t20;
        let t85 = t84 * t24;
        let t86 = t29 * rho[ip];
        let t88 = 1.0 / t30 / t86;
        let t89 = t28 * t88;
        let t94 = t46 * t29;
        let t96 = 1.0 / t18 / t94;
        let t97 = t45 * t96;
        let t99 = t43 * t97 / 54.0;
        let t100 = -t25 * t89 / 9.0 - t99;
        let t101 = t100 * t63;
        let t103 = t62 * t62;
        let t104 = 1.0 / t103;
        let t105 = t53 * t104;
        let t106 = t58 * rho[ip];
        let t107 = 1.0 / t106;
        let t110 = -t99 - t57 * t107 / 72.0;
        let t111 = t70 * t110;
        let t113 = t83 * t40;
        let t114 = t113 * t42;
        let t119 = 0.0007389300411522634 * t114 * t97 + t67 * t89 / 9.0;
        let t121 = -0.00886716049382716 * t85 * t89 + t101 * t70 - t105 * t111 + t64 * t119;
        let t126 = piecewise3(t2, 0.0, -t6 * t78 * t72 / 8.0 - 3.0 / 8.0 * t6 * t19 * t121);
        let tvrho0 = 2.0 * rho[ip] * t126 + 2.0 * t76;
        vrho[ip] += tvrho0;
        let t129 = t24 * t27;
        let t130 = t129 * t32;
        let t137 = sigma[ip] * t26 * t49;
        let t139 = t43 * t137 / 144.0;
        let t140 = t25 * t27 * t32 / 24.0 + t139;
        let t141 = t140 * t63;
        let t143 = t55 * t44;
        let t146 = t139 + t143 * t59 / 192.0;
        let t147 = t70 * t146;
        let t153 = -0.00027709876543209876 * t114 * t137 - t66 * t130 / 24.0;
        let t155 = 0.0033251851851851854 * t84 * t130 + t141 * t70 - t105 * t147 + t64 * t153;
        let t159 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t155);
        let tvsigma0 = 2.0 * rho[ip] * t159;
        vsigma[ip] += tvsigma0;
    }
}
