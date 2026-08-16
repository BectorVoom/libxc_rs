//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 698/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk698<F: Float>(t126: F, t6939: F, t102: F, t786: F, t2530: F, t2207: F, t2446: F, t875: F, t2614: F, t442: F, t2462: F, t883: F) -> (F, F, F, F, F, F) {
    let t6940 = t6939 * t126;
    let t6942 = t102 * t786;
    let t6943 = t2530 * t6942;
    let t6948 = t2207 * t126;
    let t6951 = t2446 * t102 * t875;
    let t7029 = t2614 * t442;
    let t7053 = t2462 * t883;
    (t6940, t6943, t6948, t6951, t7029, t7053)
}
