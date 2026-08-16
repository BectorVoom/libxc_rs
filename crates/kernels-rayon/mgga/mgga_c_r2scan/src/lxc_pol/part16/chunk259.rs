//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 259/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk259(t783: f64, t785: f64, t788: f64, t279: f64, t509: f64, t516: f64, t523: f64, t527: f64, t531: f64, t535: f64, t540: f64, t549: f64, t562: f64, t566: f64, t568: f64, t574: f64, t576: f64, t776: f64, t782: f64) -> (f64, f64) {
    let t791 = 0.58218257753910989057e-2_f64 * t783 * t785 * t788;
    let t792 = -t509 + t516 - t523 - 0.54878743191129263322e-1_f64 * t527 * t531 - 0.27439371595564631661e-1_f64 * t535 * t540 - 0.43341108700271342816e-1_f64 * t549 * t562 - 0.13002332610081402845e0_f64 * t566 * t568 - 0.43341108700271342816e-1_f64 * t574 * t576 + 0.43341108700271342816e-1_f64 * t776 * t279 - t782 + t791;
    (t791, t792)
}
