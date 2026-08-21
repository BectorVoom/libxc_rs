//! GGA_X_Q2D vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q2d_vxc_unpol(
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
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
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
        let t30 = t19 * t19;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 0.804 + 5.0 / 972.0 * t34;
        let t39 = 1.804 - 0.646416 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t19 / t47;
        let t53 = 100.0 - t43 * t45 * t49 / 288.0;
        let t55 = 1.0 / t22;
        let t56 = t40 * t55;
        let t57 = rmath::sqrt(sigma[ip]);
        let t60 = 1.0 / t19 / rho[ip];
        let t62 = t56 * t57 * t26 * t60;
        let t63 = rmath::pow(t62, 3.5);
        let t65 = 1.0 + t34 / 24.0;
        let t68 = t39 * t53 + 8.715382969798257e-05 * t63 * t65;
        let t70 = t21 * t21;
        let t71 = 1.0 / t70;
        let t72 = t44 * sigma[ip];
        let t74 = t46 * t46;
        let t75 = 1.0 / t74;
        let t78 = 100.0 + t71 * t72 * t75 / 576.0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t68 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
        let t84 = 1.0 / t30;
        let t89 = t36 * t36;
        let t90 = 1.0 / t89;
        let t92 = t90 * t20 * t24;
        let t93 = t29 * rho[ip];
        let t95 = 1.0 / t30 / t93;
        let t100 = t39 * t40;
        let t101 = t100 * t42;
        let t102 = t46 * t29;
        let t104 = 1.0 / t19 / t102;
        let t108 = rmath::pow(t62, 2.5);
        let t110 = t108 * t65 * t40;
        let t111 = t55 * t57;
        let t114 = t26 / t19 / t29;
        let t118 = t63 * t20;
        let t119 = t118 * t24;
        let t123 = -0.00886716049382716 * t92 * t28 * t95 * t53 + t101 * t45 * t104 / 54.0 - 0.00040671787192391866 * t110 * t111 * t114 - 9.683758855331397e-06 * t119 * t28 * t95;
        let t131 = t3 / t4 / t70 * t17;
        let t133 = 1.0 / t30 / t74;
        let t134 = t133 * t68;
        let t135 = t78 * t78;
        let t136 = 1.0 / t135;
        let t137 = t136 * t72;
        let t142 = piecewise3(t2, 0.0, -t18 * t84 * t68 * t79 / 8.0 - 3.0 / 8.0 * t18 * t19 * t123 * t79 - t131 * t134 * t137 / 192.0);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t83;
        vrho[ip] += tvrho0;
        let t145 = t27 * t32;
        let t149 = sigma[ip] * t26;
        let t153 = 1.0 / t57;
        let t154 = t55 * t153;
        let t155 = t26 * t60;
        let t159 = t24 * t27;
        let t163 = 0.0033251851851851854 * t92 * t145 * t53 - t101 * t149 * t49 / 144.0 + 0.0001525192019714695 * t110 * t154 * t155 + 3.6314095707492738e-06 * t118 * t159 * t32;
        let t168 = t46 * t93;
        let t170 = 1.0 / t30 / t168;
        let t171 = t170 * t68;
        let t172 = t136 * t44;
        let t177 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t163 * t79 + t131 * t171 * t172 / 512.0);
        let tvsigma0 = 2.0 * rho[ip] * t177;
        vsigma[ip] += tvsigma0;
    }
}
