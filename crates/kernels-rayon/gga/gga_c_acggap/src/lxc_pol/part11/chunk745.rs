//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 745/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk745(t1184: f64, t7822: f64, t1190: f64, t579: f64, t839: f64, t336: f64, t2046: f64, t1165: f64, t604: f64, t930: f64, t2068: f64, t599: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7823 = t7822 * t1184;
    let t7825 = t7822 * t1190;
    let t7827 = t579 * t839;
    let t7828 = t336 * t7827;
    let t7829 = t2046 * t7828;
    let t7832 = t1165 * t604 * t930;
    let t7833 = t2068 * t7832;
    let t7835 = t599 * t945;
    (t7823, t7825, t7828, t7829, t7832, t7833, t7835)
}
