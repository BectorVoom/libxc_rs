//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 860/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk860<F: Float>(t1043: F, t357: F, t999: F, t1012: F, t1014: F, t3252: F, t354: F, t3298: F, t378: F) -> (F, F, F, F, F, F) {
    let t4900 = t1043 * t357;
    let t4910 = t357 * t999;
    let t4915 = t1012 * t1014;
    let t4919 = t1012 * t3252;
    let t4975 = t354 * t357;
    let t4976 = t4975 * t999;
    let t4980 = t3298 * t378;
    (t4900, t4910, t4915, t4919, t4976, t4980)
}
