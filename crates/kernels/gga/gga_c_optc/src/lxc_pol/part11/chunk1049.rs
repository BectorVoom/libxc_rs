//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1049/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1049<F: Float>(t1382: F, t4565: F, t2672: F, t17235: F, t2367: F, t930: F, t17045: F, t297: F, t17180: F, t2586: F, t953: F, t17185: F, t913: F, t10959: F, t17134: F, t2812: F) -> (F, F, F, F, F, F, F) {
    let t51636 = t4565 * t1382;
    let t51645 = t1382 * t2672;
    let t51701 = t930 * t2367 * t17235;
    let t51706 = t17045 * t297;
    let t51729 = t953 * t2586 * t17180;
    let t51733 = t913 * t2367 * t17185;
    let t51736 = t2812 * t10959 * t17134;
    (t51636, t51645, t51701, t51706, t51729, t51733, t51736)
}
