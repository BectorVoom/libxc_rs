//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1091/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1091(t11880: f64, t11885: f64, t11890: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11938: f64, t11940: f64, t11941: f64, t11943: f64, t11952: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64, t9243: f64) -> f64 {
    let t11954 = -t9243 + 8.0_f64 / 27.0_f64 * t9221 + 2.0_f64 / 27.0_f64 * t9223 - 2.0_f64 / 9.0_f64 * t9226 - t9228 / 9.0_f64 + 4.0_f64 / 27.0_f64 * t11938 + t11940 - t11941 - t11943 + 10.0_f64 / 27.0_f64 * t11880 - 4.0_f64 / 3.0_f64 * t11885 - 4.0_f64 / 9.0_f64 * t11890 - 2.0_f64 / 9.0_f64 * t11896 + 2.0_f64 * t11899 + 4.0_f64 / 3.0_f64 * t11904 + 2.0_f64 / 3.0_f64 * t11908 + t11952 / 3.0_f64;
    t11954
}
