//! GGA_K_MEYER exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_meyer_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = 1.0 - t29 * t32 * t35 / 864.0;
        let t40 = t24 * t24;
        let t41 = 1.0 / t26;
        let t42 = t40 * t41;
        let t43 = f64::sqrt(sigma[ip]);
        let t44 = t43 * t30;
        let t45 = t21 * rho[ip];
        let t46 = 1.0 / t45;
        let t49 = t42 * t44 * t46 / 72.0;
        let t50 = 1.0 + t49;
        let t51 = 1.0 - t49;
        let t52 = f64::abs(t51);
        let t53 = 1.0 / t52;
        let t55 = f64::ln(t50 * t53);
        let t57 = t39 * t55 * t24;
        let t58 = 1.0 / t43;
        let t59 = t26 * t58;
        let t60 = t31 * t45;
        let t63 = 3.0 / 2.0 * t57 * t59 * t60;
        let t64 = 1.0 / 2.0 - t63;
        let t65 = 1.0 / 2.0 + t63;
        let t66 = 1.0 / t65;
        let t69 = 20.0 * t64 * t66 + 1.0;
        let t73 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t69);
        let tzk0 = 2.0 * t73;
        zk[ip] += tzk0;
    }
}
