//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 809/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk809(t12: f64, t176: f64, t7616: f64, t166: f64, t161: f64, t2389: f64, t764: f64, t7300: f64, t44: f64, t131: f64, t178: f64, t7549: f64, t7553: f64, t7557: f64, t7561: f64, t7565: f64, t7569: f64, t7571: f64, t7573: f64, t7576: f64, t7579: f64, t7583: f64, t7587: f64, t7589: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t7617 = t7616 * t176;
    let t7618 = t166 * t7617;
    let t7620 = t161 * t7618 / 30.0_f64;
    let t7621 = t764 * t2389;
    let t7626 = piecewise3(t13, 0.0_f64, 2.0_f64 * t12 * t7300 + 6.0_f64 * t7621);
    let t7627 = t7626 * t44;
    let t7628 = t7627 * t131;
    let t7630 = t7628 * t178 / 30.0_f64;
    let t7631 = t7549 - t7553 - t7557 + t7561 + t7565 + t7569 + t7571 + t7573 + t7576 + t7579 + t7583 + t7587 + t7589 + t7620 + t7630;
    (t7617, t7618, t7620, t7621, t7627, t7628, t7630, t7631)
}
