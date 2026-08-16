//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1163/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1163(t1014: f64, t42340: f64, t42341: f64, t23508: f64, t360: f64, t3127: f64, t3131: f64, t10474: f64, t10482: f64, t43288: f64, t43292: f64, t10163: f64, t386: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43503 = t42340 * t42341 * t1014;
    let t43505 = t23508 * t360;
    let t43515 = t42340 * t42341 * t3127;
    let t43516 = t23508 * t3131;
    let t43553 = t42340 * t42341 * t10474;
    let t43554 = t23508 * t10482;
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43603 = 1.0_f64 / t10163 / t386;
    (t43503, t43505, t43515, t43516, t43553, t43554, t43576, t43577, t43603)
}
