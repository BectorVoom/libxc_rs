//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1161/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1161(t17239: f64, t2367: f64, t930: f64, t17249: f64, t2569: f64, t10838: f64, t16921: f64, t4038: f64, t17317: f64, t993: f64, t16901: f64, t999: f64) -> (f64, f64, f64, f64, f64) {
    let t52154 = t930 * t2367 * t17239;
    let t52200 = t17249 * t2569;
    let t52241 = t4038 * t10838 * t16921;
    let t52245 = t17317 * t993;
    let t52260 = t999 * t2367 * t16901;
    (t52154, t52200, t52241, t52245, t52260)
}
