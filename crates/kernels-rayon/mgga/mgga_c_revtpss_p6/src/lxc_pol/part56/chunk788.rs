//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 788/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk788(t218: f64, t25273: f64, t816: f64, t228: f64, t7021: f64, t802: f64, t7043: f64, t826: f64, t2736: f64, t7082: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64, f64) {
    let t25275 = t25273 * t218 * t816;
    let t25276 = 35.0_f64 / 432.0_f64 * t25275;
    let t25277 = t7021 * t228;
    let t25278 = t25277 * t802;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25284 = 0.50820002809285328225e-5_f64 * t25283;
    let t25295 = t7082 * t72;
    let t25296 = t25295 * t686;
    (t25276, t25277, t25278, t25284, t25296)
}
