//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 841/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk841<F: Float>(t11937: F, t3205: F, t11200: F, t225: F, t366: F, t11202: F, t373: F, t371: F, t372: F, t1053: F, t3204: F, t127: F, t3218: F, t1025: F, t1058: F, t3191: F) -> (F, F, F, F, F, F, F) {
    let t11938 = t3205 * t11937;
    let t11940 = t11200 * t225;
    let t11941 = t11940 * t366;
    let t11942 = t373 * t11202;
    let t11944 = t371 * t372 * t11942;
    let t11947 = t3204 * t1053;
    let t11951 = t371 * t127 * t3218;
    let t11952 = t1025 * t11951;
    let t11954 = t3191 * t1058;
    (t11938, t11940, t11941, t11944, t11947, t11952, t11954)
}
