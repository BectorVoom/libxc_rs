//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1045/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1045<F: Float>(t16990: F, t7386: F, t888: F, t3813: F, t4768: F, t16979: F, t41818: F, t10959: F, t16636: F, t3835: F, t16644: F, t8143: F, t17047: F, t874: F, t4772: F, t16980: F, t2678: F, t40480: F) -> (F, F, F, F, F, F, F, F) {
    let t50874 = t7386 * t888 * t16990;
    let t50937 = t3813 * t4768;
    let t50941 = t41818 * t16979;
    let t50955 = t3835 * t10959 * t16636;
    let t50985 = t3835 * t8143 * t16644;
    let t50994 = t874 * t888 * t17047;
    let t51027 = t3813 * t4772;
    let t51035 = t2678 * t40480 * t16980;
    (t50874, t50937, t50941, t50955, t50985, t50994, t51027, t51035)
}
