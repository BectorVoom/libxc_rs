//! GGA_K_OL2 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_ol2_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
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
        let t24 = param_bb * sigma[ip];
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t30 = t26 * t29;
        let t33 = rmath::sqrt(sigma[ip]);
        let t34 = param_cc * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = 4.0 * t33 * t25 * t36 + t25;
        let t42 = 1.0 / t41;
        let t43 = t25 * t36 * t42;
        let t45 = param_aa + 0.013888888888888888 * t24 * t30 + t34 * t43;
        let t49 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t51 = t20 / t21;
        let t55 = t27 * rho[ip];
        let t57 = 1.0 / t22 / t55;
        let t58 = t26 * t57;
        let t62 = 1.0 / t21 / t27;
        let t64 = t25 * t62 * t42;
        let t67 = param_cc * sigma[ip];
        let t68 = t41 * t41;
        let t69 = 1.0 / t68;
        let t70 = t58 * t69;
        let t73 = -0.037037037037037035 * t24 * t58 - 4.0 / 3.0 * t34 * t64 + 16.0 / 3.0 * t67 * t70;
        let t78 = piecewise3(t2, 0.0, t7 * t51 * t45 / 10.0 + 3.0 / 20.0 * t7 * t23 * t73);
        let tvrho0 = 2.0 * rho[ip] * t78 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t81 = param_bb * t26;
        let t84 = 1.0 / t33;
        let t85 = param_cc * t84;
        let t88 = param_cc * t26;
        let t92 = 0.013888888888888888 * t81 * t29 + t85 * t43 / 2.0 - 2.0 * t88 * t29 * t69;
        let t96 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t92);
        let tvsigma0 = 2.0 * rho[ip] * t96;
        vsigma[ip] += tvsigma0;
        let t99 = t20 * t36;
        let t106 = t27 * t27;
        let t108 = 1.0 / t22 / t106;
        let t109 = t26 * t108;
        let t113 = 1.0 / t21 / t55;
        let t115 = t25 * t113 * t42;
        let t118 = t109 * t69;
        let t121 = t33 * sigma[ip];
        let t122 = param_cc * t121;
        let t123 = t106 * t27;
        let t124 = 1.0 / t123;
        let t126 = 1.0 / t68 / t41;
        let t127 = t124 * t126;
        let t130 = 0.13580246913580246 * t24 * t109 + 28.0 / 9.0 * t34 * t115 - 80.0 / 3.0 * t67 * t118 + 1024.0 / 9.0 * t122 * t127;
        let t135 = piecewise3(t2, 0.0, -t7 * t99 * t45 / 30.0 + t7 * t51 * t73 / 5.0 + 3.0 / 20.0 * t7 * t23 * t130);
        let tv2rho20 = 2.0 * rho[ip] * t135 + 4.0 * t78;
        v2rho2[ip] += tv2rho20;
        let t148 = t106 * rho[ip];
        let t149 = 1.0 / t148;
        let t151 = t126 * t33;
        let t154 = -0.037037037037037035 * t81 * t57 - 2.0 / 3.0 * t85 * t64 + 8.0 * t88 * t57 * t69 - 128.0 / 3.0 * param_cc * t149 * t151;
        let t159 = piecewise3(t2, 0.0, t7 * t51 * t92 / 10.0 + 3.0 / 20.0 * t7 * t23 * t154);
        let tv2rhosigma0 = 2.0 * rho[ip] * t159 + 2.0 * t96;
        v2rhosigma[ip] += tv2rhosigma0;
        let t162 = 1.0 / t121;
        let t163 = param_cc * t162;
        let t166 = 1.0 / sigma[ip];
        let t167 = param_cc * t166;
        let t168 = t30 * t69;
        let t170 = 1.0 / t106;
        let t175 = -t163 * t43 / 4.0 - t167 * t168 + 16.0 * param_cc * t170 * t126 * t84;
        let t179 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t175);
        let tv2sigma20 = 2.0 * rho[ip] * t179;
        v2sigma2[ip] += tv2sigma20;
    }
}
