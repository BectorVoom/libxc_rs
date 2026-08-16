//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3680/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3680(t300: f64, t69192: f64, t69216: f64, t69383: f64, t69422: f64, t69467: f64, t69500: f64, t69548: f64, t69595: f64, t69090: f64, t69094: f64, t69097: f64, t69099: f64, t69101: f64, t69103: f64, t69105: f64, t69107: f64, t69111: f64, t69115: f64, t69117: f64, t69569: f64) -> (f64, f64) {
    let t69599 = t300 * (t69192 + t69216 + t69383 + t69422 + t69467 + t69500 + t69548 + t69595);
    let t69600 = -t69090 + t69094 - t69097 + t69099 + t69101 - t69103 + t69105 + t69107 - t69111 - t69115 + t69117 + t69599 + t69569;
    (t69599, t69600)
}
