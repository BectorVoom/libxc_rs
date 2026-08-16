//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 646/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk646(t3237: f64, t3245: f64, t1115: f64, t2367: f64, t1162: f64, t3097: f64, t914: f64, t3088: f64, t1172: f64, t2586: f64, t1170: f64, t1152: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3246 = t3245 * t3237;
    let t3249 = t2367 * t1115;
    let t3250 = t1162 * t3249;
    let t3252 = t914 * t3097;
    let t3255 = t914 * t3088;
    let t3258 = t2586 * t1172;
    let t3259 = t1170 * t3258;
    let t3261 = t2367 * t1152;
    (t3246, t3250, t3252, t3255, t3259, t3261)
}
