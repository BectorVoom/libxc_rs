//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 534/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk534(t3921: f64, t848: f64, t1232: f64, t458: f64, t2771: f64, t4052: f64, t1212: f64, t2: f64, t2681: f64, t824: f64, t192: f64, t4129: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4210 = t848 * t3921;
    let t4213 = t458 * t1232;
    let t4215 = t2771 * t4052;
    let t4218 = t2 * t1212;
    let t4220 = t2681 * t4218 * t824;
    let t4224 = t192 * t852 * t4129;
    (t4210, t4213, t4215, t4218, t4220, t4224)
}
