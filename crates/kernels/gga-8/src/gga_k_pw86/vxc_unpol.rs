//! GGA_K_PW86 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 21 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_pw86_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (41 lines) ---
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
        let t29 = t24 / t27;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = t24 * t24;
        let t42 = t39 / t26 / t25;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t30;
        let t45 = t33 * t33;
        let t46 = t45 * rho[ip];
        let t48 = 1.0 / t21 / t46;
        let t52 = t43 * sigma[ip];
        let t53 = t45 * t45;
        let t54 = 1.0 / t53;
        let t57 = 1.0 + 0.91999999999999999998e-1 * t29 * t32 * t35 + 0.321875e-1 * t42 * t44 * t48 + 0.35645771717653941627e-5 * t52 * t54;
        let t58 = f64::powf(t57, 1.0 / 15.0);
        let t62 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t58);
        let tzk0 = 2.0 * t62;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (21 lines) ---
        let t63 = 1.0 / t21;
        let t68 = t7 * t20;
        let t69 = t58 * t58;
        let t70 = t69 * t69;
        let t72 = t70 * t70;
        let t73 = t72 * t70 * t69;
        let t74 = 1.0 / t73;
        let t75 = t22 * t74;
        let t76 = t33 * rho[ip];
        let t78 = 1.0 / t22 / t76;
        let t82 = t45 * t33;
        let t84 = 1.0 / t21 / t82;
        let t88 = t53 * rho[ip];
        let t89 = 1.0 / t88;
        let t92 = -0.24533333333333333333e0 * t29 * t32 * t78 - 0.17166666666666666667e0 * t42 * t44 * t84 - 0.28516617374123153302e-4 * t52 * t89;
        let t97 = piecewise3(t2, 0.0, t7 * t20 * t63 * t58 / 10.0 + t68 * t75 * t92 / 100.0);
        let tvrho0 = 2.0 * rho[ip] * t97 + 2.0 * t62;
        vrho[ip] += tvrho0;
        let t103 = sigma[ip] * t30;
        let t109 = 0.91999999999999999998e-1 * t29 * t31 * t35 + 0.64375e-1 * t42 * t103 * t48 + 0.10693731515296182488e-4 * t43 * t54;
        let t113 = piecewise3(t2, 0.0, t68 * t75 * t109 / 100.0);
        let tvsigma0 = 2.0 * rho[ip] * t113;
        vsigma[ip] += tvsigma0;
    }
}
