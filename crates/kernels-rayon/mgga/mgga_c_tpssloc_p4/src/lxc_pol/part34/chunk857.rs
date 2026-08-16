//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 857/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk857(t20371: f64, t20679: f64, t20692: f64, t20696: f64, t1458: f64, t6287: f64, t1774: f64, t5493: f64, t20347: f64, t510: f64, t16578: f64, t12861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20698 = t20371 + t20679 + t20692 + t20696;
    let t20702 = t6287 * t1458;
    let t20717 = t1774 * t5493;
    let t20720 = t510 * t20347;
    let t20723 = 3.0_f64 * t16578;
    let t20724 = 3.0_f64 * t12861;
    (t20698, t20702, t20717, t20720, t20723, t20724)
}
