//! GGA_K_THAKKAR vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_thakkar_vxc_unpol(
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
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = rmath::sqrt(sigma[ip]);
        let t31 = t30 * t24;
        let t33 = 1.0 / t21 / rho[ip];
        let t35 = rmath::ln(t31 * t33 + rmath::sqrt(pow_2(t31 * t33) + 1.0));
        let t36 = t33 * t35;
        let t39 = 1.0 + 0.0253 * t31 * t36;
        let t40 = 1.0 / t39;
        let t44 = M_CBRT4;
        let t49 = 2.0 * t44 * t30 * t24 * t33 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t33 * t50;
        let t54 = 1.0 + 0.0055 * t26 * t29 * t40 - 0.072 * t31 * t51;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        let t60 = t20 / t21;
        let t64 = t27 * rho[ip];
        let t66 = 1.0 / t22 / t64;
        let t70 = t39 * t39;
        let t71 = 1.0 / t70;
        let t72 = t29 * t71;
        let t74 = 1.0 / t21 / t27;
        let t75 = t74 * t35;
        let t78 = t26 * t29;
        let t79 = t78 + 1.0;
        let t80 = rmath::sqrt(t79);
        let t81 = 1.0 / t80;
        let t82 = t66 * t81;
        let t85 = -0.03373333333333333 * t31 * t75 - 0.03373333333333333 * t26 * t82;
        let t89 = t74 * t50;
        let t92 = t49 * t49;
        let t93 = 1.0 / t92;
        let t95 = t66 * t93 * t44;
        let t98 = -0.014666666666666666 * t26 * t66 * t40 - 0.0055 * t26 * t72 * t85 + 0.096 * t31 * t89 - 0.192 * t26 * t95;
        let t103 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t106 = t25 * t29;
        let t109 = 1.0 / t30;
        let t110 = t109 * t24;
        let t115 = 0.01265 * t110 * t36 + 0.01265 * t106 * t81;
        let t121 = t93 * t44;
        let t124 = 0.0055 * t106 * t40 - 0.0055 * t26 * t72 * t115 - 0.036 * t110 * t51 + 0.072 * t106 * t121;
        let t128 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
    }
}
