//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1325/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1325(t2617: f64, t2696: f64, t2693: f64, t809: f64, t597: f64, t61: f64) -> (f64, f64, f64) {
    let t9993 = t2617 * t2696;
    let t10014 = t809 * t2693;
    let t10021 = 1.0_f64 / t61 / t597;
    (t9993, t10014, t10021)
}
