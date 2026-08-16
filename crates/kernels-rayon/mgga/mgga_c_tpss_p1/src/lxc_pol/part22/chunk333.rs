//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 333/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk333(t1021: f64, t1046: f64, t1049: f64, t1054: f64, t1063: f64, t1069: f64, t1073: f64, t1082: f64, t294: f64, t421: f64, t425: f64) -> (f64, f64, f64) {
    let t1086 = t294 * (-0.310907e-1_f64 * t1049 * t421 + 1.0_f64 * t1054 * t1063 + t1021 - t1046 - 0.19751673498613801407e-1_f64 * t1069 + 0.5848223622634646207e0_f64 * t1073 * t1082);
    let t1088 = 0.19751673498613801407e-1_f64 * t294 * t1069;
    let t1089 = t294 * t425;
    (t1086, t1088, t1089)
}
