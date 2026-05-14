//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 783/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk783<F: Float>(t2965: F, t807: F, t286: F, t688: F, t796: F, t2667: F, t2714: F, t721: F, t2707: F, t772: F, t203: F, t281: F, t84: F, t985: F, t132: F, t2800: F, t2804: F) -> (F, F, F, F, F, F) {
    let t11553 = t2965 * t807;
    let t11557 = 0.21053605041484726346e2 * t286 * t688 * t796;
    let t11560 = 0.4274e0 * t721 * t2714 * t2667;
    let t11566 = 0.14246666666666666666e0 * t721 * t2707 * t772;
    let t11570 = 0.18989649058080861537e-2 * t281 * t203 * t985 * t84;
    let t11574 = 0.3684616320282908548e2 * t721 * t132 * t2800 * t2804;
    (t11553, t11557, t11560, t11566, t11570, t11574)
}
