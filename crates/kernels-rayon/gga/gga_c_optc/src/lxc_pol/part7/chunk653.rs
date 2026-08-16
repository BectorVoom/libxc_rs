//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 653/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk653(t115: f64, t2010: f64, t155: f64, t2156: f64, t635: f64, t140: f64, t2087: f64, t102: f64, t95: f64, t195: f64, t616: f64, t2548: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3491 = t2010 * t115;
    let t3500 = t155 * t2156;
    let t3501 = t3500 * t635;
    let t3519 = t2087 * t140;
    let t3539 = t95 * t102;
    let t3575 = t195 * t616;
    let t3608 = t322 * t2548;
    (t3491, t3500, t3501, t3519, t3539, t3575, t3608)
}
