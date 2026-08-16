//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 861/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk861(t1111: f64, t30147: f64, t30148: f64, t7842: f64, t7335: f64, t7583: f64, t2450: f64) -> (f64, f64, f64) {
    let t30151 = t30147 * t7842 * t30148 * t1111;
    let t30153 = t7583 * t7335;
    let t30154 = t2450 * t30153;
    (t30151, t30153, t30154)
}
