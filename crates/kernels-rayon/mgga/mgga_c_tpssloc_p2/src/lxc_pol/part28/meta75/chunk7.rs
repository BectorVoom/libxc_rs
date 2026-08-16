//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 495/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk495(t1496: f64, t1500: f64, t1512: f64, t1516: f64, t249: f64, t787: f64, t803: f64, t817: f64, t840: f64, t843: f64) -> f64 {
    let t1519 = -t803 - t787 * t1496 / 48.0_f64 + t1500 * t249 / 3072.0_f64 - t817 * t1512 / 3072.0_f64 - t840 - t843 * t1516 / 768.0_f64;
    t1519
}
