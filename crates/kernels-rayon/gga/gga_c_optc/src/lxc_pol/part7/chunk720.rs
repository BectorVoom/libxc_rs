//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 720/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk720(t127: f64, t6: f64, t6867: f64, t161: f64, t2030: f64, t2070: f64, t105: f64, t2156: f64, t635: f64, t2022: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6869 = t6 * t6867 * t127;
    let t6870 = t161 * t6869;
    let t6873 = t2030 * t2070;
    let t6875 = t105 * t2156;
    let t6876 = t6875 * t635;
    let t6877 = t2022 * t645;
    (t6869, t6870, t6873, t6875, t6876, t6877)
}
