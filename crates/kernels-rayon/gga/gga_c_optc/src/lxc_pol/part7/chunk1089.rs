//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1089/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1089(t2262: f64, t2268: f64, t22015: f64, t25: f64, t7578: f64) -> (f64, f64, f64) {
    let t23548 = 1.0_f64 / t2262 / t2268;
    let t23549 = t23548 * t22015;
    let t23551 = t25 * t7578 * t23549;
    (t23548, t23549, t23551)
}
