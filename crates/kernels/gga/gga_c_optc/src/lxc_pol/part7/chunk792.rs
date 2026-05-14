//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 792/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk792<F: Float>(t7415: F, t7474: F, t7877: F, t7929: F, t2684: F, t907: F, t2693: F, t902: F, t906: F, t317: F, t2695: F, t956: F, t2818: F, t2612: F, t871: F, t938: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7931 = t7415 + t7474 + t7877 + t7929;
    let t7935 = t2684 * t907;
    let t7939 = t902 * t2693;
    let t7946 = t906 * t906;
    let t7947 = 1.0 / t7946;
    let t7948 = t317 * t7947;
    let t7949 = t2695 * t956;
    let t7953 = t2693 * t956;
    let t7954 = t7953 * t2818;
    let t7958 = t938 * t2612 * t871;
    (t7931, t7935, t7939, t7946, t7947, t7948, t7949, t7953, t7954, t7958)
}
