//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 440/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk440(t1499: f64, t237: f64, t1464: f64, t1473: f64, t1476: f64, t225: f64, t680: f64, t705: f64, t752: f64, t760: f64, t765: f64) -> (f64, f64) {
    let t1500 = t1499 * t237;
    let t1504 = (t680 + t705 + t1464 + t1473 + t752 + t1476 - t760 - t765) * t225;
    (t1500, t1504)
}
