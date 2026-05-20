//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 679/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk679<F: Float>(t1012: F, t1014: F, t3252: F, t140: F, t1655: F, t1011: F, t1678: F, t342: F) -> (F, F, F, F) {
    let t4915 = t1012 * t1014;
    let t4919 = t1012 * t3252;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4935 = t342 * t1678;
    (t4915, t4919, t4925, t4935)
}
