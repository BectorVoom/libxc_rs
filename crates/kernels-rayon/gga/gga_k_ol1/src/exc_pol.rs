//! GGA_K_OL1 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol1.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_ol1_exc_pol(
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t39 = M_CBRT2;
        let t40 = f64::sqrt(sigma0);
        let t41 = t39 * t40;
        let t43 = 1.0 / t33 / rho0;
        let t47 = M_CBRT6;
        let t49 = M_PI * M_PI;
        let t50 = pow_1_3(t49);
        let t51 = t50 * t50;
        let t52 = 1.0 / t51;
        let t55 = 1.0 + 5.0 / 9.0 * (sigma0 * t36 / 72.0 + 0.677e-2 * t41 * t43) * t47 * t52;
        let t59 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t28 * t30 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t17;
        let t63 = piecewise5(t15, t12, t11, t16, t61 * t8);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t67 = t66 * t66;
        let t69 = piecewise3(t65, t24, t67 * t64);
        let t71 = rho1 * rho1;
        let t72 = pow_1_3(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t78 = f64::sqrt(sigma2);
        let t79 = t39 * t78;
        let t81 = 1.0 / t72 / rho1;
        let t88 = 1.0 + 5.0 / 9.0 * (sigma2 * t75 / 72.0 + 0.677e-2 * t79 * t81) * t47 * t52;
        let t92 = piecewise3(t60, 0.0, 3.0 / 20.0 * t6 * t69 * t30 * t88);
        let tzk0 = t59 + t92;
        zk[ip] += tzk0;
    }
}
