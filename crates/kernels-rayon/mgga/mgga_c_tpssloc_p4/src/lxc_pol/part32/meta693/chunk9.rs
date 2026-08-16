//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2155/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2155(t19739: f64, t22633: f64, t3807: f64, t6976: f64, t28131: f64, t81159: f64, t552: f64, t6434: f64, t1307: f64, t6637: f64, t6888: f64, t26331: f64, t26446: f64, t96964: f64) -> (f64, f64, f64, f64) {
    let t97119 = t22633 * t6976 * t19739 * t3807;
    let t97124 = t81159 * t28131;
    let t97126 = t552 * t6434;
    let t97129 = t6888 * t6637 * t97126 * t1307;
    let t97135 = t26331 * t26446 * t96964 * t1307;
    (t97119, t97124, t97129, t97135)
}
