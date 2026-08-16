//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3227/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3227<F: Float>(t1234: F, t1280: F, t1281: F, t1287: F, t17307: F, t17864: F, t17880: F, t21507: F, t21551: F, t21565: F, t24633: F, t24948: F, t24956: F, t24999: F, t3670: F, t3755: F, t3759: F, t45385: F, t45846: F, t5326: F, t5459: F, t59854: F, t6738: F, t72270: F, t72435: F, t82471: F, t83108: F, t83567: F) -> F {
    let t84917 = -F::cast_from(0.19756347548806534796e1_f64) * t59854 * t6738 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t3759 * t24633 + F::cast_from(0.39512695097613069591e1_f64) * t17307 * t21565 - F::cast_from(0.65854491829355115987e0_f64) * t83108 * t1281 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t82471 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t5326 * t21551 - F::cast_from(0.39512695097613069591e1_f64) * t45385 * t24956 + F::cast_from(0.19756347548806534796e1_f64) * t72435 * t21507 - F::cast_from(0.19756347548806534796e1_f64) * t17864 * t24999 - F::cast_from(0.19756347548806534796e1_f64) * t17880 * t24999 - F::cast_from(0.19756347548806534796e1_f64) * t72270 * t5459 + F::cast_from(0.65854491829355115987e0_f64) * t45846 * t24948 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t83567;
    t84917
}
