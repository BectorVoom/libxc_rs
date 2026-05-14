//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 926/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk926<F: Float>(t25270: F, t2751: F, t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t2707: F, t7025: F, t7043: F, t826: F, t2736: F, t2453: F, t7057: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25271 = t25270 * t2751;
    let t25273 = t2698 * t159;
    let t25275 = t25273 * t218 * t816;
    let t25277 = t7021 * t228;
    let t25278 = t25277 * t802;
    let t25280 = t7025 * t2707;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25299 = t2453 * t7057;
    (t25271, t25273, t25275, t25277, t25278, t25280, t25282, t25283, t25299)
}
