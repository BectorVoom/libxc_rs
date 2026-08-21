//! MGGA_X_BR89_EXPLICIT exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_br89_explicit_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = t16 * t19;
        let t21 = M_CBRT4;
        let t22 = M_CBRTPI;
        let t23 = t22 * t22;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = t15 * t15;
        let t28 = 1.0 / t26 / rho[ip];
        let t31 = param_gamma * tau[ip];
        let t34 = param_gamma * sigma[ip];
        let t35 = rho[ip] * rho[ip];
        let t37 = 1.0 / t26 / t35;
        let t41 = rmath::abs(lapl[ip] * t28 / 2.0 - 2.0 * t31 * t28 + t34 * t37 / 4.0);
        let t44 = t25 * t41 / 3.0 < 5e-13;
        let t45 = lapl[ip] * t25;
        let t48 = t25 * t28;
        let t51 = t25 * t37;
        let t54 = t45 * t28 / 6.0 - 2.0 / 3.0 * t31 * t48 + t34 * t51 / 12.0;
        let t55 = 0.0 < t54;
        let t56 = piecewise3(t55, 5e-13, -5e-13);
        let t57 = piecewise3(t44, t56, t54);
        let t60 = 2.0 / 3.0 * t23 / t57;
        let t61 = t60 <= 0.0;
        let t62 = -5e-13 < t60;
        let t63 = piecewise3(t62, -5e-13, t60);
        let t65 = 1.525525181200953 * t63 + 0.4576575543602858;
        let t66 = rmath::atan(t65);
        let t67 = -t66 + 0.4292036732051034;
        let t69 = t63 * t63;
        let t71 = t69 * t63;
        let t73 = t69 * t69;
        let t75 = t73 * t63;
        let t77 = 0.7566445420735584 - 2.636397787137096 * t63 + 5.474515996423288 * t69 - 12.65730812710829 * t71 + 4.125058472512136 * t73 - 30.42513395716384 * t75;
        let t78 = t67 * t77;
        let t84 = 0.4771976183772063 - 1.779981349455627 * t63 + 3.843384186230215 * t69 - 9.591205088051849 * t71 + 2.173018028591672 * t73 - 30.42513385160366 * t75;
        let t85 = 1.0 / t84;
        let t87 = 5e-13 < t60;
        let t88 = piecewise3(t87, t60, 5e-13);
        let t90 = rmath::ln(1.0 / (2.085749716493756 * t88) + rmath::sqrt(pow_2(1.0 / (2.085749716493756 * t88)) + 1.0));
        let t91 = t90 + 2.0;
        let t93 = t88 * t88;
        let t95 = t93 * t88;
        let t97 = t93 * t93;
        let t99 = t97 * t88;
        let t101 = 4.435009886795587e-05 + 0.5812865360445791 * t88 + 66.7427645159406 * t93 + 434.2678089722977 * t95 + 824.7765766052239 * t97 + 1657.965273158212 * t99;
        let t102 = t91 * t101;
        let t108 = 3.347285060926091e-05 + 0.4791793102397135 * t88 + 62.39226833857424 * t93 + 463.1481642793812 * t95 + 785.2360350104029 * t97 + 1657.962968223273 * t99;
        let t109 = 1.0 / t108;
        let t111 = piecewise3(t61, t78 * t85, t102 * t109);
        let t113 = rmath::exp(t111 / 3.0);
        let t114 = t21 * t113;
        let t115 = rmath::exp(-t111);
        let t117 = 1.0 + t111 / 2.0;
        let t118 = t115 * t117;
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t111;
        let t121 = t119 * t120;
        let t122 = t114 * t121;
        let t125 = piecewise3(t3, 0.0, -t20 * t122 / 4.0);
        let tzk0 = 2.0 * t125;
        zk[ip] += tzk0;
    }
}
