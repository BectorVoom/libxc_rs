//! GGA_X_BAYESIAN vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_bayesian_vxc_unpol(
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
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t20 * t20;
        let t35 = 1.0 / t22;
        let t36 = t34 * t35;
        let t37 = rmath::sqrt(sigma[ip]);
        let t44 = 1.0 + t36 * t37 * t27 / t18 / rho[ip] / 12.0;
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t33 * t46;
        let t50 = 0.1926 + 0.07900833333333333 * t26 * t47;
        let t51 = t46 * t50;
        let t55 = 1.0008 + t26 * t33 * t51 / 24.0;
        let t59 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t55);
        let tzk0 = 2.0 * t59;
        zk[ip] += tzk0;
        let t61 = t17 / t30;
        let t65 = t29 * rho[ip];
        let t67 = 1.0 / t30 / t65;
        let t68 = t28 * t67;
        let t72 = 1.0 / t21;
        let t73 = t37 * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * rho[ip];
        let t77 = 1.0 / t76;
        let t79 = 1.0 / t45 / t44;
        let t80 = t77 * t79;
        let t84 = t68 * t46;
        let t89 = -0.2106888888888889 * t26 * t84 + 0.2106888888888889 * t74 * t80;
        let t90 = t46 * t89;
        let t94 = -t26 * t68 * t51 / 9.0 + t74 * t80 * t50 / 9.0 + t26 * t33 * t90 / 24.0;
        let t99 = piecewise3(t2, 0.0, -t6 * t61 * t55 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t59;
        vrho[ip] += tvrho0;
        let t102 = t25 * t28;
        let t103 = t32 * t46;
        let t106 = t72 * t37;
        let t107 = 1.0 / t75;
        let t108 = t107 * t79;
        let t115 = 0.07900833333333333 * t25 * t47 - 0.07900833333333333 * t106 * t108;
        let t116 = t46 * t115;
        let t120 = t102 * t103 * t50 / 24.0 - t106 * t108 * t50 / 24.0 + t26 * t33 * t116 / 24.0;
        let t124 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t120);
        let tvsigma0 = 2.0 * rho[ip] * t124;
        vsigma[ip] += tvsigma0;
    }
}
