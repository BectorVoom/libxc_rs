//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 830/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk830(t7181: f64, t7183: f64, t7185: f64, t7189: f64, t7706: f64, t7710: f64, t7713: f64, t7716: f64, t7718: f64, t7720: f64) -> f64 {
    let t7918 = -t7181 - t7706 / 24.0_f64 - t7183 - 0.24223653656484234512e-2_f64 * t7710 - t7185 - 0.40372756094140390853e-3_f64 * t7713 + t7716 / 768.0_f64 - t7718 / 768.0_f64 - t7189 - t7720 / 192.0_f64;
    t7918
}
