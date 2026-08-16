//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1325/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1325(t1037: f64, t1056: f64, t26360: f64, t26391: f64, t26417: f64, t26450: f64, t3067: f64, t8693: f64, t8751: f64, t1102: f64, t2917: f64, t8749: f64, t8791: f64) -> (f64, f64, f64, f64) {
    let t26455 = 1.0_f64 * t1037 * (t26360 + t26391 + t26417 + t26450) * t1056;
    let t26457 = 0.2077890707925103596e3_f64 * t3067 * t8693;
    let t26459 = 0.4155781415850207192e3_f64 * t3067 * t8751;
    let t26463 = 0.62336721237753107879e3_f64 * t1102 * t8749 * t2917 * t8791;
    (t26455, t26457, t26459, t26463)
}
