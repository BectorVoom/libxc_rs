//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 920/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk920<F: Float>(t1280: F, t24633: F, t1811: F, t6628: F, t3769: F, t5464: F, t6622: F, t5332: F, t1287: F, t24739: F, t24751: F, t24704: F, t3783: F, t1794: F, t3302: F, t471: F) -> (F, F, F, F, F, F, F, F) {
    let t24964 = t1280 * t24633;
    let t24973 = t1811 * t6628;
    let t24974 = t24973 * t3769;
    let t24977 = t5464 * t6622;
    let t24978 = t5332 * t24977;
    let t24981 = t24739 * t1287;
    let t24986 = t24751 * t1287;
    let t24989 = t24704 * t1287;
    let t24994 = t24973 * t3783;
    let t24998 = t3302 * t1794 * t471;
    (t24964, t24974, t24978, t24981, t24986, t24989, t24994, t24998)
}
