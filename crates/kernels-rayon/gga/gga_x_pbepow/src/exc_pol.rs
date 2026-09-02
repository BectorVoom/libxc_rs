//! GGA_X_PBEPOW exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbepow_exc_pol(
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
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t40 = t33 * t39;
        let t42 = 0.9146457198521546 * t40 + 0.804;
        let t43 = 1.0 / t42;
        let t45 = t33 * t39 * t43;
        let t46 = rmath::pow(t45, 100.0);
        let t48 = 0.0001334414156799501 * t46 - 1.0;
        let t52 = 1.0 - 0.009146457198521547 * t33 * t39 * t48;
        let t56 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t52);
        let t57 = rho1 <= dens_threshold;
        let t58 = -t16;
        let t60 = piecewise5(t14, t11, t10, t15, t58 * t7);
        let t61 = 1.0 + t60;
        let t62 = t61 <= zeta_threshold;
        let t63 = pow_1_3(t61);
        let t65 = piecewise3(t62, t22, t63 * t61);
        let t66 = t65 * t26;
        let t67 = rho1 * rho1;
        let t68 = pow_1_3(rho1);
        let t69 = t68 * t68;
        let t71 = 1.0 / t69 / t67;
        let t72 = sigma2 * t71;
        let t73 = t33 * t72;
        let t75 = 0.9146457198521546 * t73 + 0.804;
        let t76 = 1.0 / t75;
        let t78 = t33 * t72 * t76;
        let t79 = rmath::pow(t78, 100.0);
        let t81 = 0.0001334414156799501 * t79 - 1.0;
        let t85 = 1.0 - 0.009146457198521547 * t33 * t72 * t81;
        let t89 = piecewise3(t57, 0.0, -3.0 / 8.0 * t5 * t66 * t85);
        let tzk0 = t56 + t89;
        zk[ip] += tzk0;
    }
}
