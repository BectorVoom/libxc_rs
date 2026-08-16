//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1213/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1213(t25035: f64, t893: f64, t7354: f64, t8152: f64, t862: f64, t10838: f64, t7359: f64, t2583: f64, t7879: f64, t3843: f64, t898: f64, t2649: f64, t7878: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25036 = t893 * t25035;
    let t25041 = t862 * t8152 * t7354;
    let t25044 = t862 * t10838 * t7359;
    let t25050 = t2583 * t7879;
    let t25052 = t3843 * t898;
    let t25053 = t893 * t25052;
    let t25055 = t7878 * t2649;
    (t25036, t25041, t25044, t25050, t25052, t25053, t25055)
}
