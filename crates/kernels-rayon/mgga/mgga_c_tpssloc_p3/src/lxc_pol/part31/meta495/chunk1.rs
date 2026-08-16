//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1690/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1690(t27975: f64, t72: f64, t5392: f64, t605: f64, t5399: f64, t1441: f64, t1458: f64) -> (f64, f64, f64, f64) {
    let t27976 = t72 * t27975;
    let t27979 = t605 * t5392;
    let t27982 = t605 * t5399;
    let t28002 = t1441 * t1458;
    (t27976, t27979, t27982, t28002)
}
