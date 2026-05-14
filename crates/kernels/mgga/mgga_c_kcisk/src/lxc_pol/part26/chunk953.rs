//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 953/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk953<F: Float>(t1354: F, t7710: F, t1364: F, t443: F, t8102: F, t1056: F, t8108: F, t14140: F, t7877: F, t5658: F, t5703: F, t8111: F, t3831: F, t7897: F, t8099: F, t7706: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25921 = t1354 * t7710;
    let t25922 = t25921 * t1364;
    let t25925 = t443 * t8102;
    let t25927 = t8108 * t1056;
    let t25930 = t14140 * t7877;
    let t25931 = t25930 * t1364;
    let t25934 = t5658 * t5703;
    let t25937 = t8111 * t1056;
    let t25940 = t3831 * t7897;
    let t25941 = t25940 * t1364;
    let t25944 = t8099 * t1056;
    let t25947 = t1354 * t7706;
    (t25922, t25925, t25927, t25931, t25934, t25937, t25941, t25944, t25947)
}
