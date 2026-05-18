//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 865/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk865<F: Float>(t22453: F, t9680: F, t4147: F, t6781: F, t9593: F, t6922: F, t566: F, t6816: F, t1843: F, t5920: F, t1513: F, t5891: F) -> (F, F, F, F, F, F, F) {
    let t22454 = t9680 * t22453;
    let t22466 = t6781 * t4147;
    let t22475 = t6781 * t9593;
    let t22483 = t6922 * t4147;
    let t22486 = t566 * t6816;
    let t22578 = t1843 * t5920;
    let t22589 = t5891 * t1513;
    (t22454, t22466, t22475, t22483, t22486, t22578, t22589)
}
