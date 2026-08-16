//! MGGA_C_CC vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cc_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(1.0 <= zeta_threshold, t35 * zeta_threshold, 1.0);
        let t40 = M_CBRT2;
        let t44 = (2.0 * t37 - 2.0) / (2.0 * t40 - 2.0);
        let t46 = 1.0 + 0.278125e-1 * t11;
        let t51 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t54 = 1.0 + 0.29608749977793437516e2 / t51;
        let t55 = f64::ln(t54);
        let t58 = 0.19751673498613801407e-1 * t44 * t46 * t55;
        let tzk0 = -t33 + t58;
        zk[ip] += tzk0;
        let t60 = 1.0 / t8 / rho[ip];
        let t61 = t7 * t60;
        let t63 = t5 * t61 * t31;
        let t65 = t27 * t27;
        let t66 = 1.0 / t65;
        let t67 = t13 * t66;
        let t69 = 1.0 / t14 * t2;
        let t70 = t4 * t7;
        let t71 = t70 * t60;
        let t72 = t69 * t71;
        let t74 = t5 * t61;
        let t76 = f64::sqrt(t11);
        let t77 = t76 * t2;
        let t78 = t77 * t71;
        let t83 = t21 * t6 / t22 / rho[ip];
        let t85 = -0.632975e0 * t72 - 0.29896666666666666667e0 * t74 - 0.1023875e0 * t78 - 0.82156666666666666667e-1 * t83;
        let t86 = 1.0 / t30;
        let t87 = t85 * t86;
        let t88 = t67 * t87;
        let t90 = t44 * t2;
        let t93 = t90 * t70 * t60 * t55;
        let t95 = t44 * t46;
        let t96 = t51 * t51;
        let t97 = 1.0 / t96;
        let t102 = -0.86308333333333333334e0 * t72 - 0.301925e0 * t74 - 0.5501625e-1 * t78 - 0.82785e-1 * t83;
        let t104 = 1.0 / t54;
        let t105 = t97 * t102 * t104;
        let t106 = t95 * t105;
        let tvrho0 = -t33 + t58 + rho[ip] * (0.11073470983333333333e-2 * t63 + 1.0 * t88 - 0.18311447306006545054e-3 * t93 - 0.5848223622634646207e0 * t106);
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
