//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2019/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2019(t106068: f64, t106070: f64, t106072: f64, t106074: f64, t95673: f64, t95674: f64, t95675: f64, t95678: f64, t95680: f64, t99044: f64, t99050: f64, t99058: f64, t99065: f64) -> f64 {
    let t110421 = -0.10289764348336736873e0_f64 * t106068 + 0.34299214494455789578e-1_f64 * t106070 - 0.85748036236139473944e-3_f64 * t106072 + 0.81312004494856525159e-4_f64 * t99044 - t95673 - 35.0_f64 / 54.0_f64 * t99050 - 0.85748036236139473944e-3_f64 * t106074 - t95674 + t95675 - t99058 + t95678 - t95680 - t99065;
    t110421
}
