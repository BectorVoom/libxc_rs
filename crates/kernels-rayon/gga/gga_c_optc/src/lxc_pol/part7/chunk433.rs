//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 433/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk433(t2036: f64, t2126: f64, t127: f64, t2067: f64, t5: f64, t675: f64, t2114: f64, t2002: f64, t56: f64) -> (f64, f64, f64, f64) {
    let t2127 = t2126 * t2036;
    let t2131 = t5 * t2067 * t127;
    let t2132 = t675 * t2131;
    let t2135 = t2114 * t127;
    let t2136 = t675 * t2135;
    let t2139 = t2002 * t56;
    (t2127, t2132, t2136, t2139)
}
