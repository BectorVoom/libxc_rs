//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 268/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk268(t5: f64, t12: f64, t208: f64, t871: f64, t594: f64, t760: f64, t598: f64, t764: f64, t44: f64, t213: f64, t224: f64, t438: f64, t492: f64, t583: f64, t590: f64, t593: f64, t609: f64, t804: f64, t808: f64, t817: f64, t826: f64, t833: f64, t837: f64, t846: f64, t855: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t872 = t871 * t208;
    let t877 = piecewise3(t6, 0.0_f64, 8.0_f64 / 3.0_f64 * t594 * t760);
    let t880 = piecewise3(t13, 0.0_f64, 8.0_f64 / 3.0_f64 * t598 * t764);
    let t883 = (t877 / 2.0_f64 + t880 / 2.0_f64) * t44;
    let t886 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + t872 * t213 / 3.0_f64 + t583 + t590 + t593 - t883 * t224 / 15.0_f64 - t609;
    (t872, t883, t886)
}
