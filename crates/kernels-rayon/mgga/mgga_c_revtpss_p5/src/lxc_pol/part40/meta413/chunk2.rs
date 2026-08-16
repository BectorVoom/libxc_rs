//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1496/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1496(t31027: f64, t31430: f64, t31032: f64, t31434: f64, t117461: f64, t31447: f64, t2357: f64, t55: f64, t116929: f64, t8402: f64, t116926: f64, t8395: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117918 = 20.0_f64 / 9.0_f64 * t31027 * t31430;
    let t117920 = 50.0_f64 / 27.0_f64 * t31032 * t31434;
    let t117927 = t117461 * t31447;
    let t117932 = t55 * t2357;
    let t117936 = t116929 * t8402;
    let t117938 = t116926 * t8395;
    (t117918, t117920, t117927, t117932, t117936, t117938)
}
