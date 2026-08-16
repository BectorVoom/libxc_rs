//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 819/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk819<F: Float>(t137: F, t7735: F, t132: F, t2650: F, t802: F, t3026: F, t5650: F, t5656: F, t5658: F, t7194: F, t7205: F, t7713: F, t7717: F, t7721: F, t7723: F, t7728: F, t7730: F, t7732: F, t7734: F) -> (F, F, F, F) {
    let t7736 = t137 * t7735;
    let t7738 = t132 * t7736 / F::cast_from(10.0_f64);
    let t7744 = t802 * t2650 / F::cast_from(10.0_f64);
    let t7745 = t7713 + t7717 + t7721 + t7723 + t3026 + F::cast_from(0.3246312408709453_f64) * t7194 + t7728 + t7730 - t7732 - t7734 - t7738 + F::cast_from(0.03354522822333102_f64) * t5650 + F::cast_from(0.6492624817418906_f64) * t5656 + F::cast_from(0.21642082724729686_f64) * t5658 + F::cast_from(4.0_f64) * t7205 - t7744;
    (t7736, t7738, t7744, t7745)
}
