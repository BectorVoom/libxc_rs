//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1626/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1626(t3961: f64, t605: f64, t3967: f64, t1433: f64, t645: f64, t72: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64) {
    let t26073 = t605 * t3961;
    let t26076 = t605 * t3967;
    let t26090 = t72 * t1433 * t645;
    let t26114 = t649 * t1458;
    (t26073, t26076, t26090, t26114)
}
