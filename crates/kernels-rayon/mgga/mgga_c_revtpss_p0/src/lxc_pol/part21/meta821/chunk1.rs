//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3039/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3039(t3298: f64, t4743: f64, t1024: f64, t11247: f64, t11788: f64, t11902: f64, t12105: f64, t15648: f64, t15655: f64, t16414: f64, t16485: f64, t16569: f64, t1692: f64, t3278: f64, t3288: f64, t3291: f64, t3295: f64, t3305: f64, t3317: f64, t3318: f64, t53670: f64, t53877: f64, t55880: f64, t55934: f64, t55938: f64, t55939: f64, t55944: f64, t55948: f64) -> f64 {
    let t55958 = t4743 * t3298;
    let t55966 = -0.19756347548806534796e1_f64 * t15655 * t3295 + 0.39512695097613069591e1_f64 * t11788 * t16485 - 0.39512695097613069591e1_f64 * t55934 * t3288 + 0.92196288561097162379e1_f64 * t55938 * t53670 * t55939 * t11247 + 0.19756347548806534796e1_f64 * t55944 * t16569 + 0.19756347548806534796e1_f64 * t55948 * t16569 + 0.39512695097613069591e1_f64 * t3278 * t16414 - 0.19756347548806534796e1_f64 * t1024 * t3291 * t15648 - 0.39512695097613069591e1_f64 * t53877 * t12105 + 0.39512695097613069591e1_f64 * t55958 * t3305 + 0.65854491829355115987e0_f64 * t11902 * t1692 - 0.19756347548806534796e1_f64 * t3317 * t55880 * t3318;
    t55966
}
