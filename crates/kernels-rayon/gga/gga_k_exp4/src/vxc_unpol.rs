//! GGA_K_EXP4 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_exp4_vxc_unpol(
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
        let t39 = f64::exp(-0.83254166666666666664e1 * t29 * t32 * t35);
        let t41 = t24 * t24;
        let t43 = 1.0 / t26 / t25;
        let t44 = t41 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t47 = t33 * t33;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t21 / t48;
        let t54 = f64::exp(-0.15095833333333333333e-1 * t44 * t45 * t30 * t50);
        let t56 = 0.20788e1 - 0.8524e0 * t39 - 0.12264e1 * t54;
        let t60 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t56);
        let tzk0 = 2.0 * t60;
        zk[ip] += tzk0;
        let t62 = t20 / t21;
        let t66 = t29 * sigma[ip];
        let t67 = t33 * rho[ip];
        let t71 = t31 / t22 / t67 * t39;
        let t74 = t44 * t45;
        let t75 = t47 * t33;
        let t77 = 1.0 / t21 / t75;
        let t78 = t30 * t77;
        let t79 = t78 * t54;
        let t82 = -0.1892422711111111111e2 * t66 * t71 - 0.98738826666666666664e-1 * t74 * t79;
        let t87 = piecewise3(t2, 0.0, t7 * t62 * t56 / 10.0 + 3.0 / 20.0 * t7 * t23 * t82);
        let tvrho0 = 2.0 * rho[ip] * t87 + 2.0 * t60;
        vrho[ip] += tvrho0;
        let t94 = t44 * sigma[ip];
        let t95 = t30 * t50;
        let t96 = t95 * t54;
        let t99 = 0.70965851666666666664e1 * t29 * t31 * t35 * t39 + 0.37027059999999999999e-1 * t94 * t96;
        let t103 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
    }
}
