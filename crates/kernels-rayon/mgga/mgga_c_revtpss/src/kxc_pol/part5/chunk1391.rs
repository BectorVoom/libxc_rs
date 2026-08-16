//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1391/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1391(t1320: f64, t6801: f64, t189: f64, t21931: f64, t512: f64, t6800: f64, t749: f64, t13611: f64, t13621: f64, t9398: f64, t9406: f64, t13630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22191 = t1320 * t6801;
    let t22192 = 4.0_f64 * t22191;
    let t22193 = t21931 * t189;
    let t22194 = t512 * t22193;
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    let t22197 = 0.11696447245269292414e1_f64 * t13611;
    let t22198 = 16.0_f64 * t13621;
    let t22199 = 8.0_f64 * t9398;
    let t22200 = 8.0_f64 * t9406;
    let t22201 = 0.23392894490538584828e1_f64 * t13630;
    (t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201)
}
