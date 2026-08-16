//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1378/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1378(t1162: f64, t3093: f64, t7274: f64, t1897: f64, t3105: f64, t9073: f64, t25423: f64, t9168: f64, t1135: f64, t8914: f64, t1027: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27422 = t1162 * t7274 * t3093;
    let t27424 = t1897 * t3105;
    let t27425 = t27424 * t9073;
    let t27438 = t9168 * t25423;
    let t27439 = t1135 * t8914;
    let t27441 = t19 * t1027;
    (t27422, t27424, t27425, t27438, t27439, t27441)
}
