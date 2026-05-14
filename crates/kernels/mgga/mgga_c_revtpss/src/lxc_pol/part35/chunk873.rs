//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 873/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk873<F: Float>(t1668: F, t6258: F, t1045: F, t3117: F, t1651: F, t6299: F, t6305: F, t3155: F, t3162: F, t11765: F, t22688: F, t1012: F, t23598: F, t373: F, t371: F, t372: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23992 = t6258 * t1668;
    let t23993 = t23992 * t1045;
    let t23994 = t3117 * t23993;
    let t23997 = t1651 * t6299;
    let t23998 = t23997 * t1045;
    let t23999 = t3117 * t23998;
    let t24007 = t1651 * t6305;
    let t24008 = t24007 * t3155;
    let t24009 = t3117 * t24008;
    let t24012 = t24007 * t3162;
    let t24013 = t3117 * t24012;
    let t24016 = t11765 * t22688;
    let t24017 = t1012 * t24016;
    let t24022 = t373 * t23598;
    let t24024 = t371 * t372 * t24022;
    (t23992, t23994, t23997, t23999, t24007, t24009, t24013, t24017, t24024)
}
