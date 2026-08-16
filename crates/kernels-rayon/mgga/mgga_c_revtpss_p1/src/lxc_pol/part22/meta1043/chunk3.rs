//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3649/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3649(t56176: f64, t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t68363: f64, t68366: f64) -> f64 {
    let t69011 = -0.14833333333333333333e0_f64 * t68363 + 0.41203703703703703704e-1_f64 * t68366 - 0.10987654320987654321e-1_f64 * t56176 + 0.32962962962962962963e-1_f64 * t56183 - 0.24722222222222222222e-1_f64 * t56185 - 0.12361111111111111111e-1_f64 * t56187 - 0.37083333333333333333e-1_f64 * t56189 + 0.82407407407407407408e-2_f64 * t56209 + 0.41203703703703703704e-2_f64 * t56212 + 0.24722222222222222223e-1_f64 * t56214 - 0.68672839506172839507e-2_f64 * t56216 + 0.16481481481481481482e-1_f64 * t56228;
    t69011
}
