//! GGA_K_OL1 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_ol1_vxc_unpol(
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
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t32 = rmath::sqrt(sigma[ip]);
        let t33 = t25 * t32;
        let t35 = 1.0 / t21 / rho[ip];
        let t39 = M_CBRT6;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = 1.0 / t43;
        let t47 = 1.0 + 5.0 / 9.0 * (t26 * t29 / 72.0 + 0.00677 * t33 * t35) * t39 * t44;
        let t51 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = 1.0 / t21;
        let t57 = t7 * t20;
        let t58 = t27 * rho[ip];
        let t60 = 1.0 / t22 / t58;
        let t64 = 1.0 / t21 / t27;
        let t67 = -t26 * t60 / 27.0 - 0.009026666666666667 * t33 * t64;
        let t69 = t39 * t44;
        let t74 = piecewise3(t2, 0.0, t7 * t20 * t52 * t47 / 10.0 + t57 * t22 * t67 * t69 / 12.0);
        let tvrho0 = 2.0 * rho[ip] * t74 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t80 = t25 / t32;
        let t83 = t25 * t29 / 72.0 + 0.003385 * t80 * t35;
        let t88 = piecewise3(t2, 0.0, t57 * t22 * t83 * t69 / 12.0);
        let tvsigma0 = 2.0 * rho[ip] * t88;
        vsigma[ip] += tvsigma0;
    }
}
