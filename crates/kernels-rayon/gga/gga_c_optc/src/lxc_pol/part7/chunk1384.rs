//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1384/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1384(t25423: f64, t4434: f64, t19: f64, t27439: f64, t1113: f64, t1122: f64, t1028: f64, t8470: f64, t27481: f64, t3133: f64, t9128: f64, t1162: f64, t2367: f64, t8538: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27552 = t4434 * t25423;
    let t27553 = t27439 * t19;
    let t27557 = t1113 * t1122;
    let t27559 = t8470 * t1028;
    let t27567 = t9128 * t27481 * t3133;
    let t27570 = t1162 * t2367 * t8538;
    (t27552, t27553, t27557, t27559, t27567, t27570)
}
