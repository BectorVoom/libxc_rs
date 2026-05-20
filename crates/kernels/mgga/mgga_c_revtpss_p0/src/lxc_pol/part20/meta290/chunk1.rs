//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1158/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1158<F: Float>(t12166: F, t342: F, t11631: F, t12051: F, t12048: F, t1024: F, t1083: F, t1087: F, t11782: F, t12111: F, t12116: F, t12119: F, t12122: F, t12124: F, t12127: F, t12128: F, t12133: F, t12137: F, t12143: F, t12146: F, t12149: F, t12150: F, t12154: F, t12157: F, t12160: F, t12163: F, t3204: F, t3223: F, t3287: F, t3288: F, t3292: F, t3295: F, t3305: F, t3319: F, t4981: F) -> (F, F, F, F) {
    let t12167 = t342 * t12166;
    let t12168 = t12051 * t11631;
    let t12169 = t12048 * t12168;
    let t12172 = -F::cast_from(0.19756347548806534796e1_f64) * t3223 * t3295 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t12111 - F::cast_from(0.19756347548806534796e1_f64) * t11782 * t1083 + F::cast_from(0.39512695097613069591e1_f64) * t12116 * t3305 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t12119 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t12124 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t12128 + F::cast_from(0.39512695097613069591e1_f64) * t4981 * t12133 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t12137 - F::cast_from(0.39512695097613069591e1_f64) * t3223 * t3292 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t12143 - F::cast_from(0.39512695097613069591e1_f64) * t12146 * t3288 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t12150 - F::cast_from(0.39512695097613069591e1_f64) * t12154 * t3288 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t12157 - F::cast_from(0.19756347548806534796e1_f64) * t12160 * t3319 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t12163 + F::cast_from(0.39512695097613069591e1_f64) * t12167 * t12169;
    (t12167, t12168, t12169, t12172)
}
