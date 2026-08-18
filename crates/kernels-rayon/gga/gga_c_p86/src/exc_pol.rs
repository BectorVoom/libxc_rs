//! GGA_C_P86 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86_exc_pol(
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
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = f64::sqrt(t11);
        let t17 = 1.0 + 0.52645 * t14 + 0.08335 * t11;
        let t20 = f64::ln(t12);
        let t23 = t4 * t10 * t20;
        let t27 = piecewise3(t13, -0.1423 / t17, 0.0311 * t20 - 0.048 + 0.0005 * t23 - 0.0029 * t11);
        let t30 = 1.0 + 0.69905 * t14 + 0.065275 * t11;
        let t37 = piecewise3(t13, -0.0843 / t30, 0.01555 * t20 - 0.0269 + 0.000175 * t23 - 0.0012 * t11);
        let t38 = t37 - t27;
        let t39 = rho0 - rho1;
        let t40 = 1.0 / t7;
        let t41 = t39 * t40;
        let t42 = 1.0 + t41;
        let t43 = t42 <= zeta_threshold;
        let t44 = pow_1_3(zeta_threshold);
        let t45 = t44 * zeta_threshold;
        let t46 = pow_1_3(t42);
        let t47 = t46 * t42;
        let t48 = piecewise3(t43, t45, t47);
        let t49 = 1.0 - t41;
        let t50 = t49 <= zeta_threshold;
        let t51 = pow_1_3(t49);
        let t52 = t51 * t49;
        let t53 = piecewise3(t50, t45, t52);
        let t54 = t48 + t53 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t38 * t54 * t59;
        let t62 = sigma0 + 2.0 * sigma1 + sigma2;
        let t63 = t7 * t7;
        let t65 = 1.0 / t8 / t63;
        let t66 = t62 * t65;
        let t67 = param_aa + param_bb;
        let t68 = param_ftilde * t67;
        let t69 = param_malpha * t1;
        let t70 = t3 * t6;
        let t71 = t70 * t9;
        let t74 = t1 * t1;
        let t75 = param_mbeta * t74;
        let t76 = t3 * t3;
        let t77 = t76 * t5;
        let t78 = t8 * t8;
        let t79 = 1.0 / t78;
        let t80 = t77 * t79;
        let t83 = param_bb + t69 * t71 / 4.0 + t75 * t80 / 4.0;
        let t84 = param_mgamma * t1;
        let t87 = param_mdelta * t74;
        let t92 = 1.0 + t84 * t71 / 4.0 + t87 * t80 / 4.0 + 2387.32414637843 * param_mbeta * t40;
        let t93 = 1.0 / t92;
        let t95 = t83 * t93 + param_aa;
        let t96 = 1.0 / t95;
        let t97 = f64::sqrt(t62);
        let t98 = t96 * t97;
        let t99 = f64::powf(t7, 1.0 / 6.0);
        let t101 = 1.0 / t99 / t7;
        let t104 = f64::exp(-t68 * t98 * t101);
        let t105 = t66 * t104;
        let t106 = t44 * t44;
        let t107 = t106 * zeta_threshold;
        let t108 = t46 * t46;
        let t109 = t108 * t42;
        let t110 = piecewise3(t43, t107, t109);
        let t111 = t51 * t51;
        let t112 = t111 * t49;
        let t113 = piecewise3(t50, t107, t112);
        let t114 = t110 + t113;
        let t115 = f64::sqrt(t114);
        let t116 = 1.0 / t115;
        let t117 = t95 * t116;
        let t118 = M_SQRT2;
        let t119 = t117 * t118;
        let t120 = t105 * t119;
        let tzk0 = t27 + t60 + t120;
        zk[ip] += tzk0;
    }
}
