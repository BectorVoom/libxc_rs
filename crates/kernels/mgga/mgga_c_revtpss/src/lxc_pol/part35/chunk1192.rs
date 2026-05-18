//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1192/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1192<F: Float>(t22822: F, t7271: F, t22815: F, t1903: F, t6874: F, t1882: F, t543: F, t6918: F, t6844: F, t6862: F, t6895: F, t196: F, t197: F, t22758: F) -> (F, F, F, F, F, F, F, F) {
    let t114584 = t7271 * t22822;
    let t114586 = t7271 * t22815;
    let t114621 = t6874 * t1903;
    let t114636 = t6918 * t1882 * t543;
    let t114640 = t6844 * t1903;
    let t114660 = t6862 * t1903;
    let t114666 = t6895 * t1882 * t543;
    let t114752 = t22758 * t196 * t197;
    (t114584, t114586, t114621, t114636, t114640, t114660, t114666, t114752)
}
