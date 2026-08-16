//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 493/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk493(t5338: f64, t5353: f64, t277: f64, t1392: f64, t500: f64, t4066: f64, t4069: f64, t1535: f64, t446: f64, t4085: f64, t4114: f64, t4116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5354 = t5338 + t5353;
    let t5355 = t277 * t5354;
    let t5372 = t500 * t1392;
    let t5375 = 48.0_f64 * t4066;
    let t5376 = 80.0_f64 * t4069;
    let t5377 = t1535 * t446;
    let t5380 = 0.21687162600603479684e-1_f64 * t4085;
    let t5381 = 40.0_f64 * t4114;
    let t5382 = 12.0_f64 * t4116;
    (t5354, t5355, t5372, t5375, t5376, t5377, t5380, t5381, t5382)
}
