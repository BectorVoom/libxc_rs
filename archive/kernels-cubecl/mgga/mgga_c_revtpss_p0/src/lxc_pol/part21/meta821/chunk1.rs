//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3039/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3039<F: Float>(t3298: F, t4743: F, t1024: F, t11247: F, t11788: F, t11902: F, t12105: F, t15648: F, t15655: F, t16414: F, t16485: F, t16569: F, t1692: F, t3278: F, t3288: F, t3291: F, t3295: F, t3305: F, t3317: F, t3318: F, t53670: F, t53877: F, t55880: F, t55934: F, t55938: F, t55939: F, t55944: F, t55948: F) -> F {
    let t55958 = t4743 * t3298;
    let t55966 = -F::cast_from(0.19756347548806534796e1_f64) * t15655 * t3295 + F::cast_from(0.39512695097613069591e1_f64) * t11788 * t16485 - F::cast_from(0.39512695097613069591e1_f64) * t55934 * t3288 + F::cast_from(0.92196288561097162379e1_f64) * t55938 * t53670 * t55939 * t11247 + F::cast_from(0.19756347548806534796e1_f64) * t55944 * t16569 + F::cast_from(0.19756347548806534796e1_f64) * t55948 * t16569 + F::cast_from(0.39512695097613069591e1_f64) * t3278 * t16414 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t3291 * t15648 - F::cast_from(0.39512695097613069591e1_f64) * t53877 * t12105 + F::cast_from(0.39512695097613069591e1_f64) * t55958 * t3305 + F::cast_from(0.65854491829355115987e0_f64) * t11902 * t1692 - F::cast_from(0.19756347548806534796e1_f64) * t3317 * t55880 * t3318;
    t55966
}
