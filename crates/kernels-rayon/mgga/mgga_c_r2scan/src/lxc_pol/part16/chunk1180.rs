//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1180/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1180(t10710: f64, t10768: f64, t29126: f64, t43083: f64, t43086: f64, t43088: f64, t43090: f64, t43092: f64, t43094: f64, t43097: f64, t43100: f64, t43103: f64, t43105: f64, t43108: f64) -> f64 {
    let t43111 = t10768 * t10710 * t29126;
    let t43113 = -0.10401866088065122276e1_f64 * t43083 + 0.43341108700271342816e-1_f64 * t43086 - 0.27439371595564631661e-1_f64 * t43088 - 0.54878743191129263322e-1_f64 * t43090 + 0.54878743191129263322e-1_f64 * t43092 + 0.86682217400542685632e-1_f64 * t43094 - 0.47609969197673950971e-2_f64 * t43097 + 0.23804984598836975486e-2_f64 * t43100 - 0.14282990759302185292e-1_f64 * t43103 + 0.47609969197673950971e-2_f64 * t43105 - 0.5200933044032561138e0_f64 * t43108 + 0.23804984598836975486e-2_f64 * t43111;
    t43113
}
