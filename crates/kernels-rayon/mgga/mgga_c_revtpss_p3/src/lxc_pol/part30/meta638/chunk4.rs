//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2212/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2212(t101214: f64, t2122: f64, t101172: f64, t101176: f64, t101182: f64, t101187: f64, t101399: f64, t2123: f64, t25162: f64, t26749: f64, t26755: f64, t26792: f64, t28105: f64, t28109: f64, t7566: f64, t7706: f64, t96792: f64, t96810: f64) -> f64 {
    let t104226 = t2122 * t101214;
    let t104249 = -10.0_f64 * t26792 * t101399 - 10.0_f64 / 3.0_f64 * t25162 * t104226 - 5.0_f64 / 3.0_f64 * t96810 * t7706 + 5.0_f64 / 6.0_f64 * t96792 * t7706 + 5.0_f64 / 3.0_f64 * t26755 * t28105 + 5.0_f64 / 3.0_f64 * t26755 * t28109 + 5.0_f64 / 3.0_f64 * t26749 * t28105 + 5.0_f64 / 6.0_f64 * t7566 * t101172 + 5.0_f64 / 3.0_f64 * t7566 * t101176 + 5.0_f64 / 3.0_f64 * t26749 * t28109 + 5.0_f64 / 6.0_f64 * t7566 * t101182 + t101187 * t2123 / 3.0_f64;
    t104249
}
