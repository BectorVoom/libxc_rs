//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 415/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk415(t174: f64, t176: f64, t2641: f64, t2642: f64, t2645: f64, t2639: f64, t44: f64, t844: f64, t88: f64, t194: f64, t843: f64, t189: f64, t850: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t2649 = piecewise3(t175, 0.0_f64, 4.0_f64 / 9.0_f64 * t2641 * t2642 + 4.0_f64 / 3.0_f64 * t176 * t2645);
    let t2651 = (t2639 + t2649) * t44;
    let t2658 = t88 * t844;
    let t2662 = t843 * t194;
    let t2663 = 1.0_f64 / t2662;
    let t2664 = t189 * t2663;
    let t2665 = t850 * t850;
    (t2651, t2658, t2663, t2664, t2665)
}
