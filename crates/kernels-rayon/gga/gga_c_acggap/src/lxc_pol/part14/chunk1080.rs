//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1080/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1080(t1849: f64, t30148: f64, t30154: f64, t7842: f64, t30937: f64, t9608: f64, t1181: f64, t5527: f64, t7564: f64, t8600: f64, t24196: f64, t336: f64, t570: f64) -> (f64, f64, f64, f64) {
    let t38976 = t30154 * t7842 * t30148 * t1849;
    let t38978 = t30937 * t9608;
    let t38982 = t7564 * t1181 * t8600 * t5527;
    let t38986 = t570 * t336 * t24196;
    (t38976, t38978, t38982, t38986)
}
