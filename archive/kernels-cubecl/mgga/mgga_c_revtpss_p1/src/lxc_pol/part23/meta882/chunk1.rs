//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2793/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793<F: Float>(t2782: F, t47371: F, t75047: F, t1398: F, t6862: F, t10022: F, t22315: F, t46457: F, t136: F, t2457: F, t47429: F, t10014: F, t22332: F) -> (F, F, F, F, F) {
    let t75049 = t2782 * t47371 * t75047;
    let t75051 = t6862 * t1398;
    let t75053 = t2782 * t10022 * t75051;
    let t75060 = t46457 * t22315;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75071 = t10014 * t22332;
    (t75049, t75053, t75060, t75068, t75071)
}
