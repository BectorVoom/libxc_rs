//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 299/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk299(t279: f64, t509: f64, t516: f64, t523: f64, t527: f64, t535: f64, t549: f64, t566: f64, t574: f64, t782: f64, t791: f64, t924: f64, t928: f64, t940: f64, t944: f64, t948: f64, t980: f64) -> f64 {
    let t983 = -t509 + t516 - t523 - 0.54878743191129263322e-1_f64 * t527 * t924 - 0.27439371595564631661e-1_f64 * t535 * t928 - 0.43341108700271342816e-1_f64 * t549 * t940 - 0.13002332610081402845e0_f64 * t566 * t944 - 0.43341108700271342816e-1_f64 * t574 * t948 + 0.43341108700271342816e-1_f64 * t980 * t279 - t782 + t791;
    t983
}
