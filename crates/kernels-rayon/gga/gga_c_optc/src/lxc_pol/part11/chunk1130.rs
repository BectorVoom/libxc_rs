//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1130/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1130(t16537: f64, t7122: f64, t16546: f64, t9917: f64, t16406: f64, t7110: f64, t16505: f64, t2120: f64, t16487: f64, t16438: f64, t2182: f64, t16416: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48962 = t7122 * t16537;
    let t48990 = t9917 * t16546;
    let t48992 = t7110 * t16406;
    let t49019 = t2120 * t16505;
    let t49023 = t2120 * t16487;
    let t49035 = t2182 * t16438;
    let t49046 = t2182 * t16416;
    (t48962, t48990, t48992, t49019, t49023, t49035, t49046)
}
