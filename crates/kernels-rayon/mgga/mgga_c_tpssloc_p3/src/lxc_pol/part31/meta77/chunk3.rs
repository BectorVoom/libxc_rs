//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 494/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk494(t1464: f64, t1473: f64, t1476: f64, t1484: f64, t1530: f64, t193: f64, t202: f64, t680: f64, t705: f64, t752: f64, t760: f64, t765: f64, t766: f64, t870: f64) -> f64 {
    let t1534 = t1530 * t193 * t202 * t870 + 3.0_f64 * t1484 * t193 * t766 + t1464 + t1473 + t1476 + t680 + t705 + t752 - t760 - t765;
    t1534
}
