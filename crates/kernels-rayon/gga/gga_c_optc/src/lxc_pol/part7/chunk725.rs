//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 725/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk725(t6915: f64, t136: f64, t141: f64, t6856: f64, t131: f64, t6165: f64, t130: f64, t142: f64, t127: f64, t2067: f64, t616: f64, t2034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6916 = 1.0_f64 / t6915;
    let t6917 = t136 * t6916;
    let t6919 = t6917 * t141 * t6856;
    let t6922 = t131 * t6165;
    let t6923 = t130 * t6922;
    let t6925 = 0.47892880429854730775e0_f64 * t6923 * t142;
    let t6926 = t2067 * t127;
    let t6927 = t6926 * t616;
    let t6928 = t2034 * t6927;
    (t6916, t6917, t6919, t6922, t6923, t6925, t6926, t6927, t6928)
}
