//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 684/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk684(t1981: f64, t576: f64, t579: f64, t922: f64, t336: f64, t2020: f64, t374: f64, t1145: f64, t2041: f64, t1117: f64, t1121: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7400 = t576 * t1981;
    let t7401 = t579 * t922;
    let t7402 = t336 * t7401;
    let t7403 = t7400 * t7402;
    let t7405 = t2020 * t374;
    let t7406 = 7.0_f64 / 144.0_f64 * t7405;
    let t7407 = t2041 * t1145;
    let t7409 = t2041 * t1117;
    let t7411 = t570 * t1121;
    (t7402, t7403, t7405, t7406, t7407, t7409, t7411)
}
