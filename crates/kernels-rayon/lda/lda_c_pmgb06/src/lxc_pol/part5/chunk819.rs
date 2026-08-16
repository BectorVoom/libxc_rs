//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 819/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk819(t137: f64, t7735: f64, t132: f64, t2650: f64, t802: f64, t3026: f64, t5650: f64, t5656: f64, t5658: f64, t7194: f64, t7205: f64, t7713: f64, t7717: f64, t7721: f64, t7723: f64, t7728: f64, t7730: f64, t7732: f64, t7734: f64) -> (f64, f64, f64, f64) {
    let t7736 = t137 * t7735;
    let t7738 = t132 * t7736 / 10.0_f64;
    let t7744 = t802 * t2650 / 10.0_f64;
    let t7745 = t7713 + t7717 + t7721 + t7723 + t3026 + 0.3246312408709453_f64 * t7194 + t7728 + t7730 - t7732 - t7734 - t7738 + 0.03354522822333102_f64 * t5650 + 0.6492624817418906_f64 * t5656 + 0.21642082724729686_f64 * t5658 + 4.0_f64 * t7205 - t7744;
    (t7736, t7738, t7744, t7745)
}
