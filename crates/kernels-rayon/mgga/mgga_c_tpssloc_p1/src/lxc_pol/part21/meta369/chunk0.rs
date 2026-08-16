//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1811/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1811(t13563: f64, t13566: f64, t4348: f64, t690: f64) -> (f64, f64, f64) {
    let t13600 = 4.0_f64 / 27.0_f64 * t13563;
    let t13601 = 4.0_f64 / 9.0_f64 * t13566;
    let t13602 = t690 * t4348;
    (t13600, t13601, t13602)
}
