//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 835/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk835(t2004: f64, t38638: f64, t38594: f64, t38599: f64, t38604: f64, t38606: f64, t38608: f64, t38610: f64, t38615: f64, t38617: f64, t38619: f64, t38623: f64, t38624: f64, t38626: f64, t38628: f64, t38630: f64, t38632: f64, t38634: f64, t38636: f64) -> f64 {
    let t38639 = t38638 * t2004;
    let t38640 = 0.19863479950205658386e-4_f64 * t38639;
    let t38641 = -0.25538759935978703638e-4_f64 * t38594 + 0.25538759935978703638e-4_f64 * t38599 + 0.85129199786595678796e-5_f64 * t38604 - 0.85129199786595678796e-5_f64 * t38606 - 0.15243824895787514157e-3_f64 * t38608 + 0.15243824895787514157e-3_f64 * t38610 - 0.85129199786595678796e-5_f64 * t38615 - 0.42564599893297839398e-5_f64 * t38617 + 0.85129199786595678796e-5_f64 * t38619 + t38623 + 0.12769379967989351819e-4_f64 * t38624 - 0.25538759935978703638e-4_f64 * t38626 - 0.25538759935978703638e-4_f64 * t38628 - 0.12769379967989351819e-4_f64 * t38630 - 0.85129199786595678796e-5_f64 * t38632 - 0.85129199786595678796e-5_f64 * t38634 - 0.42564599893297839398e-5_f64 * t38636 + t38640;
    t38641
}
