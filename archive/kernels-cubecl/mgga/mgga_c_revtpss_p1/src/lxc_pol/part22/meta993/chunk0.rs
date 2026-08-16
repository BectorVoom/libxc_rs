//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3379/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3379<F: Float>(t141: F, t63297: F, t930: F, t2908: F, t63364: F, t63283: F, t11341: F, t63288: F, t63449: F, t2439: F, t6132: F, t63455: F) -> (F, F, F, F, F, F, F) {
    let t63519 = t141 * t930 * t63297;
    let t63522 = t141 * t2908 * t63364;
    let t63525 = t141 * t2908 * t63283;
    let t63528 = t141 * t11341 * t63288;
    let t63531 = t141 * t930 * t63449;
    let t63533 = t2439 * t6132;
    let t63536 = t141 * t2908 * t63455;
    (t63519, t63522, t63525, t63528, t63531, t63533, t63536)
}
