//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2250/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2250(t4237: f64, t644: f64, t77: f64, t1497: f64, t2311: f64, t4241: f64, t640: f64, t13420: f64, t84: f64, t25099: f64, t25106: f64, t28086: f64, t28090: f64, t28105: f64, t28109: f64, t6958: f64, t6963: f64, t7706: f64, t92644: f64, t92702: f64) -> f64 {
    let t101156 = t77 * t4237 * t644;
    let t101172 = t77 * t2311 * t1497;
    let t101176 = t77 * t640 * t4241;
    let t101182 = t77 * t84 * t13420;
    let t101185 = 2.0_f64 / 3.0_f64 * t6963 * t28086 + 5.0_f64 / 3.0_f64 * t6958 * t101156 + 2.0_f64 / 3.0_f64 * t6963 * t28090 + 5.0_f64 / 3.0_f64 * t92702 * t7706 + 5.0_f64 / 6.0_f64 * t92644 * t7706 + 5.0_f64 / 3.0_f64 * t25106 * t28105 + 5.0_f64 / 3.0_f64 * t25106 * t28109 + 5.0_f64 / 3.0_f64 * t25099 * t28105 + 5.0_f64 / 6.0_f64 * t6958 * t101172 + 5.0_f64 / 3.0_f64 * t6958 * t101176 + 5.0_f64 / 3.0_f64 * t25099 * t28109 + 5.0_f64 / 6.0_f64 * t6958 * t101182;
    t101185
}
