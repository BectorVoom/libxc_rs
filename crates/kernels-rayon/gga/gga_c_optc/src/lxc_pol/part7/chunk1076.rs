//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1076/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1076(t714: f64, t7142: f64, t108: f64, t176: f64, t203: f64, t616: f64, t6599: f64, t1948: f64, t2226: f64, t6560: f64, t729: f64, t1972: f64) -> (f64, f64, f64, f64, f64) {
    let t23360 = t7142 * t714;
    let t23373 = t176 * t6599 * t616 * t108 * t203;
    let t23378 = t176 * t2226 * t1948 * t108 * t203;
    let t23383 = t176 * t729 * t6560 * t108 * t203;
    let t23390 = t1972 * t1972;
    (t23360, t23373, t23378, t23383, t23390)
}
