//! GGA_C_P86VWN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86vwn.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86vwn_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = rmath::sqrt(t10);
        let t14 = t11 + 1.86372 * t12 + 12.9352;
        let t15 = 1.0 / t14;
        let t19 = rmath::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.0310907 * t19;
        let t21 = t12 + 3.72744;
        let t24 = rmath::atan(6.15199081975908 / t21);
        let t25 = 0.038783294878113016 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498;
        let t28 = t27 * t27;
        let t30 = rmath::ln(t28 * t15);
        let t31 = 0.0009690227711544374 * t30;
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.565535 * t12 + 13.0045;
        let t36 = 1.0 / t35;
        let t40 = rmath::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 1.13107;
        let t44 = rmath::atan(7.123108917818118 / t41);
        let t46 = t26 + 0.0047584;
        let t47 = t46 * t46;
        let t49 = rmath::ln(t47 * t36);
        let t53 = 1.0 <= zeta_threshold;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(t53, t54 * zeta_threshold, 1.0);
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t65 = 9.0 * t56 - 9.0;
        let t67 = t33 * (t40 + 0.31770800474394145 * t44 + 0.00041403379428206277 * t49) * t65 / 24.0;
        let t68 = rho[ip] * rho[ip];
        let t70 = 1.0 / t7 / t68;
        let t71 = sigma[ip] * t70;
        let t72 = param_aa + param_bb;
        let t73 = param_ftilde * t72;
        let t74 = param_malpha * t1;
        let t75 = t3 * t6;
        let t76 = t75 * t8;
        let t79 = t1 * t1;
        let t80 = param_mbeta * t79;
        let t81 = t3 * t3;
        let t82 = t81 * t5;
        let t83 = t7 * t7;
        let t84 = 1.0 / t83;
        let t85 = t82 * t84;
        let t88 = param_bb + t74 * t76 / 4.0 + t80 * t85 / 4.0;
        let t89 = param_mgamma * t1;
        let t92 = param_mdelta * t79;
        let t95 = 1.0 / rho[ip];
        let t98 = 1.0 + t89 * t76 / 4.0 + t92 * t85 / 4.0 + 2387.32414637843 * param_mbeta * t95;
        let t99 = 1.0 / t98;
        let t101 = t88 * t99 + param_aa;
        let t102 = 1.0 / t101;
        let t103 = rmath::sqrt(sigma[ip]);
        let t104 = t102 * t103;
        let t105 = rmath::pow(rho[ip], 1.0 / 6.0);
        let t107 = 1.0 / t105 / rho[ip];
        let t110 = rmath::exp(-t73 * t104 * t107);
        let t112 = t54 * t54;
        let t114 = piecewise3(t53, t112 * zeta_threshold, 1.0);
        let t115 = rmath::sqrt(t114);
        let t116 = 1.0 / t115;
        let t117 = t110 * t101 * t116;
        let t118 = t71 * t117;
        let tzk0 = t20 + t25 + t31 - t67 + t118;
        zk[ip] += tzk0;
    }
}
