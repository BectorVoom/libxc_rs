//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1026/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1026(t115275: f64, t115277: f64, t115279: f64, t115283: f64, t115666: f64, t115669: f64, t115672: f64, t115674: f64, t115676: f64, t115678: f64, t117533: f64, t2040: f64, t2323: f64, t23929: f64, t31832: f64, t32350: f64, t3652: f64, t672: f64, t7156: f64, t7218: f64, t7264: f64, t7266: f64, t8329: f64, t85428: f64, t8829: f64) -> f64 {
    let t117604 = -4.0_f64 * t117533 * t672 - 2.0_f64 * t2040 * t85428 - 4.0_f64 * t2323 * t32350 - 4.0_f64 * t23929 * t7266 + 2.0_f64 * t31832 * t7218 - t3652 * t8829 - 2.0_f64 * t7156 * t7264 - t115275 - t115277 - t115279 + t115283 + t115666 - t115669 - t115672 - t115674 - t115676 + t115678 - t8329;
    t117604
}
