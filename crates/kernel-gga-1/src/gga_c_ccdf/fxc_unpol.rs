//! GGA_C_CCDF fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ccdf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT6, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_ccdf_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_c3: f64,
    param_c4: f64,
    param_c5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = param_c2 * t2 + 1.0;
        let t5 = 1.0 / t4;
        let t6 = param_c1 * t5;
        let t7 = M_CBRT2;
        let t8 = M_CBRT6;
        let t9 = t8 * t8;
        let t10 = t7 * t9;
        let t11 = M_PI * M_PI;
        let t12 = pow_1_3(t11);
        let t13 = 1.0 / t12;
        let t14 = f64::sqrt(sigma[ip]);
        let t15 = t13 * t14;
        let t17 = 1.0 / t1 / rho[ip];
        let t23 = f64::exp(-param_c4 * (t10 * t15 * t17 / 12.0 - param_c5));
        let t24 = 1.0 + t23;
        let t27 = 1.0 - param_c3 / t24;
        let tzk0 = t6 * t27;
        zk[ip] += tzk0;
        let t28 = t2 * param_c1;
        let t29 = t4 * t4;
        let t30 = 1.0 / t29;
        let t36 = t5 * param_c3;
        let t37 = t24 * t24;
        let t38 = 1.0 / t37;
        let t39 = t36 * t38;
        let t40 = t17 * param_c1 * t39;
        let t42 = param_c4 * t7 * t9;
        let tvrho0 = tzk0 + t28 * t30 * t27 * param_c2 / 3.0 + t40 * t42 * t15 * t23 / 9.0;
        vrho[ip] += tvrho0;
        let t47 = t28 * t39;
        let t48 = 1.0 / t14;
        let t51 = t42 * t13 * t48 * t23;
        let tvsigma0 = -t47 * t51 / 24.0;
        vsigma[ip] += tvsigma0;
        let t54 = param_c1 * t30;
        let t55 = t27 * param_c2;
        let t59 = param_c3 * t38;
        let t60 = t59 * param_c4;
        let t61 = t6 * t60;
        let t62 = t10 * t13;
        let t63 = rho[ip] * rho[ip];
        let t65 = 1.0 / t1 / t63;
        let t71 = t1 * t1;
        let t74 = 1.0 / t71 / rho[ip] * param_c1;
        let t76 = 1.0 / t29 / t4;
        let t78 = param_c2 * param_c2;
        let t83 = 1.0 / t71 / t63;
        let t84 = t83 * param_c1;
        let t86 = t84 * t30 * t60;
        let t87 = t14 * t23;
        let t88 = t87 * param_c2;
        let t92 = t63 * rho[ip];
        let t94 = 1.0 / t71 / t92;
        let t95 = t94 * param_c1;
        let t97 = 1.0 / t37 / t24;
        let t98 = t36 * t97;
        let t99 = t95 * t98;
        let t100 = param_c4 * param_c4;
        let t101 = t7 * t7;
        let t102 = t100 * t101;
        let t103 = t102 * t8;
        let t104 = t12 * t12;
        let t105 = 1.0 / t104;
        let t106 = t105 * sigma[ip];
        let t107 = t23 * t23;
        let t112 = t95 * t39;
        let tv2rho20 = 2.0 / 9.0 * t54 * t55 * t17 - t61 * t62 * t14 * t65 * t23 / 27.0 + 2.0 / 9.0 * t74 * t76 * t27 * t78 + 2.0 / 27.0 * t86 * t62 * t88 - 4.0 / 27.0 * t99 * t103 * t106 * t107 + 2.0 / 27.0 * t112 * t103 * t106 * t23;
        v2rho2[ip] += tv2rho20;
        let t120 = t74 * t30 * t60;
        let t121 = t48 * t23;
        let t122 = t121 * param_c2;
        let t123 = t62 * t122;
        let t126 = t84 * t98;
        let t127 = t8 * t105;
        let t129 = t102 * t127 * t107;
        let t132 = t84 * t39;
        let t134 = t102 * t127 * t23;
        let tv2rhosigma0 = t40 * t51 / 72.0 - t120 * t123 / 72.0 + t126 * t129 / 18.0 - t132 * t134 / 36.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = t74 * t98;
        let t138 = 1.0 / sigma[ip];
        let t139 = t105 * t138;
        let t141 = t103 * t139 * t107;
        let t144 = t14 * sigma[ip];
        let t145 = 1.0 / t144;
        let t148 = t42 * t13 * t145 * t23;
        let t151 = t74 * t39;
        let t153 = t103 * t139 * t23;
        let tv2sigma20 = -t137 * t141 / 48.0 + t47 * t148 / 48.0 + t151 * t153 / 96.0;
        v2sigma2[ip] += tv2sigma20;
    }
}
