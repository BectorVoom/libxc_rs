//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 915/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk915(t1041: f64, t10459: f64, t1008: f64, t349: f64, t1011: f64) -> (f64, f64, f64, f64) {
    let t10460 = t1041 * t10459;
    let t10468 = t1008 * t1008;
    let t10469 = 1.0_f64 / t10468;
    let t10470 = t349 * t10469;
    let t10471 = t1011 * t1011;
    (t10460, t10469, t10470, t10471)
}
