//! GGA_C_P86 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86_exc_unpol(
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
        let t12 = 1.0 <= t11;
        let t13 = f64::sqrt(t10);
        let t16 = 1.0 + 0.52645 * t13 + 0.08335 * t10;
        let t19 = f64::ln(t11);
        let t22 = t4 * t9 * t19;
        let t26 = piecewise3(t12, -0.1423 / t16, 0.0311 * t19 - 0.048 + 0.0005 * t22 - 0.0029 * t10);
        let t29 = 1.0 + 0.69905 * t13 + 0.065275 * t10;
        let t36 = piecewise3(t12, -0.0843 / t29, 0.01555 * t19 - 0.0269 + 0.000175 * t22 - 0.0012 * t10);
        let t38 = 1.0 <= zeta_threshold;
        let t39 = pow_1_3(zeta_threshold);
        let t41 = piecewise3(t38, t39 * zeta_threshold, 1.0);
        let t43 = 2.0 * t41 - 2.0;
        let t45 = M_CBRT2;
        let t48 = 1.0 / (2.0 * t45 - 2.0);
        let t49 = (t36 - t26) * t43 * t48;
        let t50 = rho[ip] * rho[ip];
        let t52 = 1.0 / t7 / t50;
        let t53 = sigma[ip] * t52;
        let t54 = param_aa + param_bb;
        let t55 = param_ftilde * t54;
        let t56 = param_malpha * t1;
        let t57 = t3 * t6;
        let t58 = t57 * t8;
        let t61 = t1 * t1;
        let t62 = param_mbeta * t61;
        let t63 = t3 * t3;
        let t64 = t63 * t5;
        let t65 = t7 * t7;
        let t66 = 1.0 / t65;
        let t67 = t64 * t66;
        let t70 = param_bb + t56 * t58 / 4.0 + t62 * t67 / 4.0;
        let t71 = param_mgamma * t1;
        let t74 = param_mdelta * t61;
        let t77 = 1.0 / rho[ip];
        let t80 = 1.0 + t71 * t58 / 4.0 + t74 * t67 / 4.0 + 2387.32414637843 * param_mbeta * t77;
        let t81 = 1.0 / t80;
        let t83 = t70 * t81 + param_aa;
        let t84 = 1.0 / t83;
        let t85 = f64::sqrt(sigma[ip]);
        let t86 = t84 * t85;
        let t87 = f64::powf(rho[ip], 1.0 / 6.0);
        let t89 = 1.0 / t87 / rho[ip];
        let t92 = f64::exp(-t55 * t86 * t89);
        let t94 = t39 * t39;
        let t96 = piecewise3(t38, t94 * zeta_threshold, 1.0);
        let t97 = f64::sqrt(t96);
        let t98 = 1.0 / t97;
        let t99 = t92 * t83 * t98;
        let t100 = t53 * t99;
        let tzk0 = t26 + t49 + t100;
        zk[ip] += tzk0;
    }
}
