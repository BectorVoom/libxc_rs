//! GGA_K_PG vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pg.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_pg_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_pg_mu: f64,
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
        let t36 = t32 * t35;
        let t40 = param_pg_mu * t24 * t28;
        let t43 = f64::exp(-t40 * t36 / 24.0);
        let t44 = 5.0 / 72.0 * t29 * t36 + t43;
        let t48 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t50 = t20 / t21;
        let t54 = t33 * rho[ip];
        let t56 = 1.0 / t22 / t54;
        let t64 = -5.0 / 27.0 * t29 * t32 * t56 + t40 * t32 * t56 * t43 / 9.0;
        let t69 = piecewise3(t2, 0.0, t7 * t50 * t44 / 10.0 + 3.0 / 20.0 * t7 * t23 * t64);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t72 = t31 * t35;
        let t78 = 5.0 / 72.0 * t29 * t72 - t40 * t72 * t43 / 24.0;
        let t82 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t78);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
    }
}
