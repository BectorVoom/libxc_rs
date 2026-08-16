//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 405/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk405(t1986: f64, t603: f64, t1847: f64, t1849: f64, t587: f64) -> (f64, f64) {
    let t1987 = t1986 * t603;
    let t1988 = 0.11696446794910408142e1_f64 * t1987;
    let t1990 = t1847 * t1849 * t587;
    (t1988, t1990)
}
