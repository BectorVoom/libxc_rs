//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 335/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk335(t1433: f64, t72: f64, t1411: f64, t1427: f64, t66: f64, t80: f64) -> (f64, f64) {
    let t1434 = t72 * t1433;
    let t1437 = -t1411 * t80 / 12.0_f64 + t1427 * t80 / 24.0_f64 + t66 * t1434 / 24.0_f64;
    (t1434, t1437)
}
