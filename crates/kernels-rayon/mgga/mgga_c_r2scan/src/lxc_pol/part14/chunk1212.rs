//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1212/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1212(t39629: f64, t39637: f64, t39640: f64, t39642: f64, t37718: f64, t37721: f64, t39632: f64, t39635: f64, t39645: f64, t39647: f64, t39650: f64, t41474: f64) -> f64 {
    let t41475 = 0.13869154784086829701e1_f64 * t39629;
    let t41478 = 0.32927245914677557993e-1_f64 * t39637;
    let t41479 = 0.65854491829355115984e-1_f64 * t39640;
    let t41480 = 0.11708928647259339622e0_f64 * t39642;
    let t41484 = -0.95219938395347901946e-2_f64 * t37718 - 0.28565981518604370584e-1_f64 * t37721 + t41474 + t41475 + 0.52396431978519890152e-1_f64 * t39632 - 0.25426783770825854453e1_f64 * t39635 - t41478 - t41479 + t41480 + 0.52009330440325611378e0_f64 * t39645 + 0.32927245914677557992e0_f64 * t39647 - 0.13099107994629972538e-1_f64 * t39650;
    t41484
}
