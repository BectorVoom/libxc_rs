//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2595/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2595<F: Float>(t18550: F, t72: F, t757: F, t18299: F, t750: F, t18298: F, t705: F, t18281: F, t706: F, t18838: F, t892: F, t2609: F, t2611: F, t5819: F) -> (F, F, F, F, F, F) {
    let t61093 = t18550 * t72 * t757;
    let t61114 = t18299 * t750;
    let t61122 = t705 * t18298;
    let t61130 = t706 * t750 * t18281;
    let t61139 = t18838 * t892;
    let t61165 = t2611 * t2609 * t5819;
    (t61093, t61114, t61122, t61130, t61139, t61165)
}
