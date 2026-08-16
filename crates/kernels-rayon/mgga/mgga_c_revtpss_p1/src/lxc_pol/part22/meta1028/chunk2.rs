//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3606/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> f64 {
    let t68443 = -16.0_f64 / 3.0_f64 * t68363 + 40.0_f64 / 27.0_f64 * t68366 - 32.0_f64 / 81.0_f64 * t56176 + 32.0_f64 / 27.0_f64 * t56183 - 8.0_f64 / 9.0_f64 * t56185 - 4.0_f64 / 9.0_f64 * t56187 - 4.0_f64 / 3.0_f64 * t56189 + 8.0_f64 / 27.0_f64 * t56209 + 4.0_f64 / 27.0_f64 * t56212 + 8.0_f64 / 9.0_f64 * t56214 - 20.0_f64 / 81.0_f64 * t56216 + 16.0_f64 / 27.0_f64 * t56228;
    t68443
}
