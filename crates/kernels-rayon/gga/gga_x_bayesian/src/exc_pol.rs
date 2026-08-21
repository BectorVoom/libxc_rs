//! GGA_X_BAYESIAN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_bayesian_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t28 * t28;
        let t41 = 1.0 / t30;
        let t42 = t40 * t41;
        let t43 = rmath::sqrt(sigma0);
        let t49 = 1.0 + t42 * t43 / t36 / rho0 / 12.0;
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t39 * t51;
        let t57 = 0.1926 + 0.07900833333333333 * t33 * sigma0 * t39 * t51;
        let t58 = t52 * t57;
        let t61 = 1.0008 + t34 * t58 / 24.0;
        let t65 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t61);
        let t66 = rho1 <= dens_threshold;
        let t67 = -t16;
        let t69 = piecewise5(t14, t11, t10, t15, t67 * t7);
        let t70 = 1.0 + t69;
        let t71 = t70 <= zeta_threshold;
        let t72 = pow_1_3(t70);
        let t74 = piecewise3(t71, t22, t72 * t70);
        let t75 = t74 * t26;
        let t76 = t33 * sigma2;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = rmath::sqrt(sigma2);
        let t88 = 1.0 + t42 * t82 / t78 / rho1 / 12.0;
        let t89 = t88 * t88;
        let t90 = 1.0 / t89;
        let t91 = t81 * t90;
        let t96 = 0.1926 + 0.07900833333333333 * t33 * sigma2 * t81 * t90;
        let t97 = t91 * t96;
        let t100 = 1.0008 + t76 * t97 / 24.0;
        let t104 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t75 * t100);
        let tzk0 = t65 + t104;
        zk[ip] += tzk0;
    }
}
