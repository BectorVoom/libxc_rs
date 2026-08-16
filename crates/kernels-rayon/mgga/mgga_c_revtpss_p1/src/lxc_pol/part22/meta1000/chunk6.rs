//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3404/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3404(t41330: f64, t41332: f64, t52047: f64, t52049: f64, t52051: f64, t63399: f64, t63447: f64, t63451: f64, t63453: f64, t63457: f64, t63459: f64, t63462: f64, t63464: f64) -> f64 {
    let t63889 = 0.82407407407407407408e-2_f64 * t52047 + 0.41203703703703703704e-2_f64 * t52049 + 0.68672839506172839507e-2_f64 * t52051 - 0.2225e0_f64 * t63399 - 0.41203703703703703703e-2_f64 * t41330 - 0.27469135802469135802e-2_f64 * t41332 + 0.61805555555555555556e-2_f64 * t63447 - 0.92708333333333333333e-2_f64 * t63451 - 0.27469135802469135803e-2_f64 * t63453 - 0.12361111111111111111e-1_f64 * t63457 + 0.82407407407407407409e-2_f64 * t63459 + 0.37083333333333333334e-1_f64 * t63462 - 0.41203703703703703704e-2_f64 * t63464;
    t63889
}
