//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1308/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1308(t232: f64, t57312: f64, t57324: f64, t57114: f64, t7681: f64, t799: f64, t16867: f64, t30827: f64, t24699: f64, t7672: f64, t10493: f64, t16875: f64) -> (f64, f64, f64, f64, f64) {
    let t57327 = 0.62182e-1_f64 * (t57312 + t57324) * t232;
    let t57330 = 24.0_f64 * t7681 * t57114 * t799;
    let t57332 = 0.38596378373162651572e3_f64 * t30827 * t16867;
    let t57335 = 0.620700176468474021e4_f64 * t24699 * t57114 * t7672;
    let t57337 = 24.0_f64 * t10493 * t16875;
    (t57327, t57330, t57332, t57335, t57337)
}
