//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 792/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk792(t2476: f64, t4919: f64, t4854: f64, t7504: f64, t4780: f64, t828: f64, t2520: f64, t4884: f64, t4868: f64, t7801: f64, t4863: f64, t809: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14091 = t4919 * t2476;
    let t14098 = t4854 * t7504;
    let t14102 = t4780 * t828;
    let t14148 = t4884 * t2520;
    let t14155 = t4868 * t7801;
    let t14235 = t4863 * t809;
    (t14091, t14098, t14102, t14148, t14155, t14235)
}
