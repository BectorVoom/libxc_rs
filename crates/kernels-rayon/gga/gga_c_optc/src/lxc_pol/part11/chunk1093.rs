//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1093/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1093(t40526: f64, t893: f64, t4979: f64, t7878: f64, t4937: f64, t530: f64, t862: f64, t2372: f64, t4895: f64, t2472: f64, t4780: f64, t2529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40527 = t893 * t40526;
    let t40538 = t7878 * t4979;
    let t40539 = t893 * t40538;
    let t40677 = t862 * t530 * t4937;
    let t40764 = t4895 * t2372;
    let t40919 = t4780 * t2472;
    let t40949 = t4780 * t2529;
    (t40527, t40538, t40539, t40677, t40764, t40919, t40949)
}
