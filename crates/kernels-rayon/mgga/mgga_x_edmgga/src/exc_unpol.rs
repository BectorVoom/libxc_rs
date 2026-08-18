//! MGGA_X_EDMGGA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_edmgga_exc_unpol(
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
        let t21 = M_CBRT4;
        let t22 = t4 * t4;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t27 = t21 * t22 * t25 / 9.0;
        let t28 = 1.0 - t27;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = tau[ip] * t30;
        let t32 = t19 * t19;
        let t34 = 1.0 / t32 / rho[ip];
        let t36 = sigma[ip] * t30;
        let t37 = rho[ip] * rho[ip];
        let t39 = 1.0 / t32 / t37;
        let t42 = lapl[ip] * t30;
        let t46 = M_CBRT6;
        let t48 = t25 * t25;
        let t49 = 1.0 / t48;
        let t50 = (t31 * t34 - t36 * t39 / 8.0 - t42 * t34 / 4.0) * t46 * t49;
        let t51 = 5.0 / 9.0 * t50;
        let t52 = -t51 < -14205.545454545454;
        let t53 = 0.39111111111111113 * t50;
        let t55 = 0.0 < 0.7041420454545455 - t53;
        let t57 = piecewise3(t55, -0.00014204545454545454, 0.704 - t53);
        let t60 = t57 * t57;
        let t61 = t60 * t57;
        let t62 = 1.0 / t61;
        let t65 = 1.0 - t51;
        let t66 = t65 * t65;
        let t68 = 1.0 + 0.495616 * t66;
        let t69 = f64::sqrt(t68);
        let t71 = piecewise3(t52, -1.0 / t57 / 2.0 + t62 / 8.0, 0.704 - t53 + t69);
        let t72 = t28 * t71;
        let t73 = f64::sqrt(30.0);
        let t74 = t28 * t73;
        let t75 = f64::sqrt(t71);
        let t76 = t28 * t28;
        let t81 = 0.6018478308354863 * t76 - 0.0206514;
        let t82 = t71 - 1.0;
        let t86 = f64::ln(0.3910293204892512 / t76 / t28 * t73 * t81 * t82 + f64::sqrt(pow_2(0.3910293204892512 / t76 / t28 * t73 * t81 * t82) + 1.0));
        let t90 = 1.0 + 0.14163895778062927 * t74 * t75 * t86;
        let t91 = 1.0 / t90;
        let t93 = t72 * t91 + t27;
        let t97 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t93);
        let tzk0 = 2.0 * t97;
        zk[ip] += tzk0;
    }
}
