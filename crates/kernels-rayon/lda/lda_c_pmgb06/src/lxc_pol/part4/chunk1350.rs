//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1350/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1350(t12555: f64, t6639: f64, t6643: f64, t12516: f64, t6646: f64, t10316: f64, t12684: f64, t13043: f64, t13296: f64, t13770: f64, t13775: f64, t13782: f64, t13807: f64, t1629: f64, t16829: f64, t17483: f64, t3290: f64, t443: f64, t477: f64, t5077: f64, t6636: f64, t6637: f64, t6638: f64, t6642: f64) -> f64 {
    let t17734 = t12555 * t6639;
    let t17736 = t12555 * t6643;
    let t17738 = t12516 * t6646;
    let t17765 = -8.0_f64 / 27.0_f64 * t13770 - 4.0_f64 / 81.0_f64 * t13775 - 4.0_f64 / 135.0_f64 * t13782 + 16.0_f64 / 135.0_f64 * t17734 + 16.0_f64 / 135.0_f64 * t17736 - 8.0_f64 / 81.0_f64 * t17738 + 8.0_f64 / 45.0_f64 * t12684 * t6639 + 8.0_f64 / 45.0_f64 * t5077 * t3290 * t443 * t6638 + 8.0_f64 / 45.0_f64 * t5077 * t6636 * t17483 * t477 + 4.0_f64 / 45.0_f64 * t5077 * t6636 * t6637 * t1629 + 16.0_f64 / 45.0_f64 * t13043 * t6636 * t16829 * t477 + 8.0_f64 / 45.0_f64 * t12684 * t6643 + 8.0_f64 / 45.0_f64 * t5077 * t13296 * t6642 + 8.0_f64 / 45.0_f64 * t13807 - 4.0_f64 / 405.0_f64 * t10316;
    t17765
}
