//! GGA_X_2D_PBE exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_pbe_exc_pol(
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
        let t2 = rmath::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = rmath::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = rmath::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3(t18, t20, t22);
        let t24 = t3 * t23;
        let t25 = M_SQRT2;
        let t26 = rmath::sqrt(t4);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t33 = 0.4604 + 0.007053485964254291 * sigma0 * t30;
        let t36 = 1.4604 - 0.21196816 / t33;
        let t37 = t27 * t36;
        let t40 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t37);
        let t41 = rho1 <= dens_threshold;
        let t42 = -t14;
        let t44 = piecewise5(t12, t9, t8, t13, t42 * t5);
        let t45 = 1.0 + t44;
        let t46 = t45 <= zeta_threshold;
        let t47 = rmath::sqrt(t45);
        let t48 = t47 * t45;
        let t49 = piecewise3(t46, t20, t48);
        let t50 = t3 * t49;
        let t51 = rho1 * rho1;
        let t52 = t51 * rho1;
        let t53 = 1.0 / t52;
        let t56 = 0.4604 + 0.007053485964254291 * sigma2 * t53;
        let t59 = 1.4604 - 0.21196816 / t56;
        let t60 = t27 * t59;
        let t63 = piecewise3(t41, 0.0, -2.0 / 3.0 * t50 * t60);
        let tzk0 = t40 + t63;
        zk[ip] += tzk0;
    }
}
