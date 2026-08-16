//! GGA_C_WI vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_wi_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = param_b * sigma[ip];
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t7 = param_k * sigma[ip];
        let t9 = f64::exp(-t7 * t6);
        let t12 = t1 * t6 * t9 + param_a;
        let t13 = M_CBRT3;
        let t15 = pow_1_3(1.0 / M_PI);
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = t17 * t17;
        let t22 = t13 * t13;
        let t23 = M_CBRTPI;
        let t25 = f64::sqrt(sigma[ip]);
        let t26 = t25 * sigma[ip];
        let t27 = t2 * t2;
        let t28 = 1.0 / t27;
        let t31 = 1.0 / t3 / rho[ip];
        let t32 = t25 * t31;
        let t33 = f64::sqrt(t32);
        let t38 = 1.0 + param_d * t17 * t22 * t23 * t33 * t26 * t28 / 3.0;
        let t42 = param_c + t16 * t18 / t3 * t38 / 4.0;
        let t43 = 1.0 / t42;
        let tzk0 = t12 * t43;
        zk[ip] += tzk0;
        let t44 = t2 * rho[ip];
        let t46 = 1.0 / t4 / t44;
        let t49 = sigma[ip] * sigma[ip];
        let t50 = param_b * t49;
        let t51 = t27 * t2;
        let t53 = 1.0 / t3 / t51;
        let t58 = 8.0 / 3.0 * t50 * t53 * param_k * t9 - 8.0 / 3.0 * t1 * t46 * t9;
        let t59 = rho[ip] * t58;
        let t61 = rho[ip] * t12;
        let t62 = t42 * t42;
        let t63 = 1.0 / t62;
        let t71 = t33 * sigma[ip] * t6;
        let t72 = t23 * t71;
        let t73 = t72 * t25;
        let t76 = -t16 * t18 * t31 * t38 / 12.0 - 14.0 / 3.0 * t15 * t6 * param_d * t73;
        let t77 = t63 * t76;
        let tvrho0 = t59 * t43 - t61 * t77 + tzk0;
        vrho[ip] += tvrho0;
        let t81 = t27 * rho[ip];
        let t83 = 1.0 / t3 / t81;
        let t87 = -t1 * t83 * param_k * t9 + param_b * t6 * t9;
        let t88 = rho[ip] * t87;
        let t90 = 1.0 / t4;
        let t91 = t90 * t12;
        let t92 = t63 * t15;
        let t93 = t91 * t92;
        let t94 = param_d * t23;
        let t95 = 1.0 / t25;
        let t96 = t71 * t95;
        let t97 = t94 * t96;
        let tvsigma0 = t88 * t43 - 7.0 / 4.0 * t93 * t97;
        vsigma[ip] += tvsigma0;
    }
}
