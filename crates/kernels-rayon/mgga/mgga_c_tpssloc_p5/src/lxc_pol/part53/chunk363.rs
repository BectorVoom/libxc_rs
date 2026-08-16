//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 363/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk363(t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t510: f64, t513: f64, t574: f64, t652: f64) -> f64 {
    let t1851 = -t113 * t1774 - t1442 * t510 - 2.0_f64 * t1459 * t652 + t1778 * t574 + t1849 * t513;
    t1851
}
