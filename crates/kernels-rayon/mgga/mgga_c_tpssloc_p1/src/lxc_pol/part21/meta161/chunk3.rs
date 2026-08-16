//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1052/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1052(t17: f64, t3826: f64, t1285: f64, t592: f64, t1287: f64) -> (f64, f64, f64, f64, f64) {
    let t3827 = t17 * t3826;
    let t3828 = 2.0_f64 * t3827;
    let t3829 = t592 * t1285;
    let t3830 = 8.0_f64 * t3829;
    let t3832 = 8.0_f64 * t592 * t1287;
    (t3827, t3828, t3829, t3830, t3832)
}
