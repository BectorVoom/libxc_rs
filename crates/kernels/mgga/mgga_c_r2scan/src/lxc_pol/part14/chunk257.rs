//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 257/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk257<F: Float>(t783: F, t785: F, t788: F, t279: F, t509: F, t516: F, t523: F, t527: F, t531: F, t535: F, t540: F, t549: F, t562: F, t566: F, t568: F, t574: F, t576: F, t776: F, t782: F) -> (F, F) {
    let t791 = 0.58218257753910989057e-2 * t783 * t785 * t788;
    let t792 = -t509 + t516 - t523 - 0.54878743191129263322e-1 * t527 * t531 - 0.27439371595564631661e-1 * t535 * t540 - 0.43341108700271342816e-1 * t549 * t562 - 0.13002332610081402845e0 * t566 * t568 - 0.43341108700271342816e-1 * t574 * t576 + 0.43341108700271342816e-1 * t776 * t279 - t782 + t791;
    (t791, t792)
}
