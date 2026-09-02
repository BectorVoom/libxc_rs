//! GGA_K_LC94 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lc94_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t22 / t34;
        let t37 = t33 * t36;
        let t40 = rmath::exp(-param_alpha * t24 * t29 * t37 / 24.0);
        let t43 = (t40 * param_d + param_c) * t24;
        let t44 = t43 * t29;
        let t47 = t24 * t24;
        let t48 = 1.0 / t27;
        let t49 = t47 * t48;
        let t50 = rmath::sqrt(sigma[ip]);
        let t53 = 1.0 / t21 / rho[ip];
        let t54 = t50 * t31 * t53;
        let t57 = rmath::pow(t49 * t54 / 12.0, param_expo);
        let t58 = param_f * t57;
        let t59 = t44 * t37 / 24.0 - t58;
        let t60 = t49 * t50;
        let t66 = rmath::ln(param_b * t47 * t48 * t54 / 12.0 + rmath::sqrt(pow_2(param_b * t47 * t48 * t54 / 12.0) + 1.0));
        let t67 = param_a * t66;
        let t68 = t31 * t53 * t67;
        let t71 = 1.0 + t60 * t68 / 12.0 + t58;
        let t72 = 1.0 / t71;
        let t74 = t59 * t72 + 1.0;
        let t78 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t20 / t21;
        let t84 = param_d * param_alpha;
        let t86 = 1.0 / t27 / t26;
        let t87 = t47 * t86;
        let t88 = t84 * t87;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t31;
        let t91 = t34 * t34;
        let t92 = t91 * t34;
        let t94 = 1.0 / t21 / t92;
        let t95 = t94 * t40;
        let t99 = t34 * rho[ip];
        let t101 = 1.0 / t22 / t99;
        let t105 = 1.0 / rho[ip];
        let t108 = 4.0 / 3.0 * t58 * param_expo * t105;
        let t109 = t88 * t90 * t95 / 108.0 - t44 * t33 * t101 / 9.0 + t108;
        let t111 = t71 * t71;
        let t112 = 1.0 / t111;
        let t113 = t59 * t112;
        let t115 = 1.0 / t21 / t34;
        let t117 = t31 * t115 * t67;
        let t120 = t24 * t29;
        let t121 = t120 * t33;
        let t123 = param_b * param_b;
        let t128 = 6.0 * t123 * t24 * t29 * t37 + 144.0;
        let t129 = rmath::sqrt(t128);
        let t131 = param_b / t129;
        let t132 = t101 * param_a * t131;
        let t135 = -t60 * t117 / 9.0 - 2.0 / 3.0 * t121 * t132 - t108;
        let t137 = t109 * t72 - t113 * t135;
        let t142 = piecewise3(t2, 0.0, t7 * t80 * t74 / 10.0 + 3.0 / 20.0 * t7 * t23 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t145 = t91 * rho[ip];
        let t147 = 1.0 / t21 / t145;
        let t148 = t31 * t147;
        let t149 = t40 * sigma[ip];
        let t153 = t29 * t32;
        let t157 = 1.0 / sigma[ip];
        let t160 = t58 * param_expo * t157 / 2.0;
        let t161 = -t88 * t148 * t149 / 288.0 + t43 * t153 * t36 / 24.0 - t160;
        let t164 = t49 / t50;
        let t167 = t120 * t32;
        let t169 = t36 * param_a * t131;
        let t172 = t164 * t68 / 24.0 + t167 * t169 / 4.0 + t160;
        let t174 = -t113 * t172 + t161 * t72;
        let t178 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t174);
        let tvsigma0 = 2.0 * rho[ip] * t178;
        vsigma[ip] += tvsigma0;
    }
}
