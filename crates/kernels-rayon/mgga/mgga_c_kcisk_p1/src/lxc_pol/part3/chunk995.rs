//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 995/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk995(t13338: f64, t13342: f64, t13344: f64, t13347: f64, t13351: f64, t13355: f64, t13359: f64, t13362: f64, t13367: f64, t13372: f64, t13375: f64, t13380: f64) -> f64 {
    let t14659 = -0.27857666666666666666e-1_f64 * t13338 + 0.69644166666666666666e-2_f64 * t13342 - 0.46429444444444444443e-2_f64 * t13344 - 0.69644166666666666666e-2_f64 * t13347 - 0.92858888888888888888e-2_f64 * t13351 - 0.15476481481481481482e-1_f64 * t13355 - 0.11607361111111111111e-1_f64 * t13359 - 0.69644166666666666665e-2_f64 * t13362 - 0.18571777777777777778e-1_f64 * t13367 + 0.11607361111111111111e-2_f64 * t13372 + 0.34822083333333333333e-2_f64 * t13375 - 0.46429444444444444443e-2_f64 * t13380;
    t14659
}
