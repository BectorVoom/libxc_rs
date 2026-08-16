//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1032/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1032<F: Float>(t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t7043: F, t826: F, t2736: F, t2453: F, t7057: F, t1954: F, t9645: F) -> (F, F, F, F, F, F, F) {
    let t25273 = t2698 * t159;
    let t25275 = t25273 * t218 * t816;
    let t25277 = t7021 * t228;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25299 = t2453 * t7057;
    let t25304 = t1954 * t9645;
    (t25273, t25275, t25277, t25282, t25283, t25299, t25304)
}
