//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3402/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3402(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64) -> f64 {
    let t63861 = -0.16481481481481481482e-1_f64 * t51973 + 0.19228395061728395062e-1_f64 * t51978 + 0.19228395061728395061e-1_f64 * t41361 + 0.82407407407407407406e-2_f64 * t41363 - 0.41203703703703703704e-1_f64 * t63325 + 0.14833333333333333333e0_f64 * t63328 + 0.2225e0_f64 * t63336 - 0.24722222222222222222e-1_f64 * t63338 + 0.82407407407407407407e-2_f64 * t63340 + 0.68672839506172839506e-2_f64 * t63342 - 0.10300925925925925926e-1_f64 * t63346 - 0.27469135802469135803e-1_f64 * t63351 + 0.37083333333333333333e-1_f64 * t63355;
    t63861
}
