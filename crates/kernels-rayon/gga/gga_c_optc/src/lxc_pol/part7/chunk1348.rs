//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1348/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1348(t26182: f64, t26234: f64, t26473: f64, t26858: f64, t1135: f64, t508: f64, t1027: f64, t11985: f64, t3116: f64, t12068: f64, t4386: f64, t8498: f64) -> (f64, f64, f64) {
    let t26860 = t26182 + t26234 + t26473 + t26858;
    let t26869 = t508 * t1135;
    let t26870 = t26869 * t1027;
    let t26872 = t3116 * t26870 * t11985;
    let t26878 = t4386 * t12068 * t8498;
    (t26860, t26872, t26878)
}
