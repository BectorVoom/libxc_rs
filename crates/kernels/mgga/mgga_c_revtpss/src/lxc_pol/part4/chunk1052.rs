//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1052/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1052<F: Float>(t11200: F, t225: F, t127: F, t3218: F, t371: F, t1025: F, t1058: F, t3191: F, t1021: F, t3201: F, t3231: F, t1054: F) -> (F, F, F, F, F, F) {
    let t11940 = t11200 * t225;
    let t11951 = t371 * t127 * t3218;
    let t11952 = t1025 * t11951;
    let t11954 = t3191 * t1058;
    let t11956 = t1021 * t3201;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    (t11940, t11952, t11954, t11956, t11965, t11967)
}
