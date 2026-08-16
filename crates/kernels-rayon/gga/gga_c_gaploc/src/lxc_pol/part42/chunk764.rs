//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 764/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk764(t1: f64, t35951: f64, t544: f64, t11425: f64, t1397: f64, t11264: f64, t524: f64, t11385: f64, t540: f64, t106: f64, t11218: f64, t192: f64) -> (f64, f64, f64, f64, f64) {
    let t37675 = t544 * t35951 * t1;
    let t37679 = t1397 * t11425;
    let t37777 = t524 * t11264;
    let t37956 = t11385 * t540;
    let t37965 = t11218 * t1 * t106 * t192;
    (t37675, t37679, t37777, t37956, t37965)
}
