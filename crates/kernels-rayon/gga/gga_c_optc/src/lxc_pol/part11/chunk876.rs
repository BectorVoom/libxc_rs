//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 876/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk876(t16671: f64, t265: f64, t1342: f64, t13890: f64, t2416: f64, t1355: f64, t4884: f64, t1354: f64, t14148: f64, t1367: f64, t4919: f64, t1366: f64, t14091: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16672 = t16671 * t265;
    let t16674 = t13890 * t1342;
    let t16676 = 0.48245472966453314466e2_f64 * t2416 * t16674;
    let t16677 = t1355 * t4884;
    let t16680 = t14148 * t1354;
    let t16683 = t1367 * t4919;
    let t16686 = t14091 * t1366;
    (t16672, t16674, t16676, t16677, t16680, t16683, t16686)
}
