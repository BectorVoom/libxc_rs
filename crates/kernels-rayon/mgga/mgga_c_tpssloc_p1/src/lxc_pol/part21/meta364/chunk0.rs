//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1792/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1792(t4351: f64, t892: f64, t914: f64, t2837: f64, t4354: f64, t1543: f64, t2841: f64) -> (f64, f64, f64, f64) {
    let t13515 = t4351 * t892;
    let t13517 = 2.0_f64 * t13515 * t914;
    let t13519 = 1.0_f64 * t4354 * t2837;
    let t13520 = t1543 * t2841;
    (t13515, t13517, t13519, t13520)
}
