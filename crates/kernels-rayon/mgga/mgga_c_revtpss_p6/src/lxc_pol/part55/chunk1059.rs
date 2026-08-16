//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1059/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1059(t7315: f64, t8714: f64, t2014: f64, t7239: f64, t8698: f64, t7235: f64, t8715: f64, t2022: f64, t7506: f64, t8707: f64, t2097: f64, t7274: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32662 = t8714 * t7315;
    let t32663 = t2014 * t32662;
    let t32667 = 3.0_f64 * t8698 * t7239;
    let t32671 = t7235 * t8715;
    let t32673 = t7506 * t2022;
    let t32674 = t8707 * t32673;
    let t32677 = t2097 * t7274;
    (t32662, t32663, t32667, t32671, t32673, t32674, t32677)
}
