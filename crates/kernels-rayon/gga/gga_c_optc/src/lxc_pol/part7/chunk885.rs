//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 885/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk885(t4387: f64, t8498: f64, t3133: f64, t8488: f64, t3132: f64, t1137: f64, t7878: f64, t1133: f64, t2586: f64, t3156: f64, t1135: f64, t2849: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8499 = t4387 * t8498;
    let t8502 = t8488 * t3133;
    let t8503 = t3132 * t8502;
    let t8505 = t7878 * t1137;
    let t8506 = t1133 * t8505;
    let t8508 = t2586 * t3156;
    let t8509 = t1133 * t8508;
    let t8511 = t1135 * t2849;
    (t8499, t8503, t8505, t8506, t8508, t8509, t8511)
}
