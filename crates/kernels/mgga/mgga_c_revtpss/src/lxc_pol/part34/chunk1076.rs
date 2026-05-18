//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1076/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1076<F: Float>(t5486: F, t6573: F, t1287: F, t1811: F, t6622: F, t13149: F, t24911: F, t6587: F, t1280: F, t24713: F, t13129: F, t1774: F, t21541: F) -> (F, F, F, F, F, F, F) {
    let t24922 = t5486 * t6573;
    let t24928 = t1811 * t6622 * t1287;
    let t24931 = t24911 * t13149;
    let t24934 = t5486 * t6587;
    let t24941 = t1280 * t24713;
    let t24948 = t24911 * t13129;
    let t24951 = t21541 * t1774;
    (t24922, t24928, t24931, t24934, t24941, t24948, t24951)
}
