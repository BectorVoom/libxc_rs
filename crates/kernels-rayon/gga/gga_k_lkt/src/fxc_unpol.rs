//! GGA_K_LKT fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lkt_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_a: f64,
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
        let t25 = t24 * t24;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t29 = t25 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = M_CBRT2;
        let t32 = t30 * t31;
        let t34 = 1.0 / t21 / rho[ip];
        let t37 = t29 * t32 * t34 / 12.0;
        let t38 = t37 < 200.0;
        let t39 = piecewise3(t38, t37, 200.0);
        let t40 = param_a * t39;
        let t41 = f64::cosh(t40);
        let t42 = 1.0 / t41;
        let t43 = t27 * t27;
        let t45 = t24 / t43;
        let t46 = t31 * t31;
        let t47 = sigma[ip] * t46;
        let t48 = rho[ip] * rho[ip];
        let t50 = 1.0 / t22 / t48;
        let t54 = t42 + 5.0 / 72.0 * t45 * t47 * t50;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        let t60 = t20 / t21;
        let t64 = t41 * t41;
        let t65 = 1.0 / t64;
        let t66 = t65 * param_a;
        let t68 = 1.0 / t21 / t48;
        let t72 = piecewise3(t38, -t29 * t32 * t68 / 9.0, 0.0);
        let t73 = f64::sinh(t40);
        let t74 = t72 * t73;
        let t76 = t48 * rho[ip];
        let t78 = 1.0 / t22 / t76;
        let t82 = -t66 * t74 - 5.0 / 27.0 * t45 * t47 * t78;
        let t87 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t82);
        let tvrho0 = 2.0 * rho[ip] * t87 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t91 = 1.0 / t30 * t31;
        let t95 = piecewise3(t38, t29 * t91 * t34 / 24.0, 0.0);
        let t96 = t95 * t73;
        let t101 = -t66 * t96 + 5.0 / 72.0 * t45 * t46 * t50;
        let t105 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t101);
        let tvsigma0 = 2.0 * rho[ip] * t105;
        vsigma[ip] += tvsigma0;
        let t108 = t20 * t34;
        let t116 = 1.0 / t64 / t41;
        let t117 = param_a * param_a;
        let t118 = t116 * t117;
        let t119 = t72 * t72;
        let t120 = t73 * t73;
        let t125 = 1.0 / t21 / t76;
        let t129 = piecewise3(t38, 7.0 / 27.0 * t29 * t32 * t125, 0.0);
        let t132 = t42 * t117;
        let t134 = t48 * t48;
        let t136 = 1.0 / t22 / t134;
        let t140 = 2.0 * t118 * t119 * t120 - t66 * t129 * t73 - t132 * t119 + 55.0 / 81.0 * t45 * t47 * t136;
        let t145 = piecewise3(t2, 0.0, -t7 * t108 * t54 / 30.0 + t7 * t60 * t82 / 5.0 + 3.0 / 20.0 * t7 * t23 * t140);
        let tv2rho20 = 2.0 * rho[ip] * t145 + 4.0 * t87;
        v2rho2[ip] += tv2rho20;
        let t151 = t95 * t120;
        let t158 = piecewise3(t38, -t29 * t91 * t68 / 18.0, 0.0);
        let t159 = t158 * t73;
        let t166 = 2.0 * t118 * t151 * t72 - t66 * t159 - t132 * t95 * t72 - 5.0 / 27.0 * t45 * t46 * t78;
        let t171 = piecewise3(t2, 0.0, t7 * t60 * t101 / 10.0 + 3.0 / 20.0 * t7 * t23 * t166);
        let tv2rhosigma0 = 2.0 * rho[ip] * t171 + 2.0 * t105;
        v2rhosigma[ip] += tv2rhosigma0;
        let t174 = t95 * t95;
        let t175 = t174 * t120;
        let t180 = 1.0 / t30 / sigma[ip] * t31;
        let t184 = piecewise3(t38, -t29 * t180 * t34 / 48.0, 0.0);
        let t185 = t184 * t73;
        let t188 = 2.0 * t118 * t175 - t132 * t174 - t66 * t185;
        let t192 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t188);
        let tv2sigma20 = 2.0 * rho[ip] * t192;
        v2sigma2[ip] += tv2sigma20;
    }
}
