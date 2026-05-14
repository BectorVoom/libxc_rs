//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 753/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk753<F: Float>(t166: F, t7725: F, t161: F, t2601: F, t831: F, t2563: F, t853: F, t2654: F, t6734: F, t822: F, t137: F, t132: F, t2650: F, t802: F, t3026: F, t5650: F, t5656: F, t5658: F, t7194: F, t7205: F, t7713: F, t7717: F, t7721: F, t7723: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7726 = t166 * t7725;
    let t7728 = t161 * t7726 / 5.0;
    let t7730 = t831 * t2601 / 5.0;
    let t7732 = t2563 * t853 / 10.0;
    let t7734 = t831 * t2654 / 5.0;
    let t7735 = t6734 * t822;
    let t7736 = t137 * t7735;
    let t7738 = t132 * t7736 / 10.0;
    let t7744 = t802 * t2650 / 10.0;
    let t7745 = t7713 + t7717 + t7721 + t7723 + t3026 + 0.3246312408709453 * t7194 + t7728 + t7730 - t7732 - t7734 - t7738 + 0.03354522822333102 * t5650 + 0.6492624817418906 * t5656 + 0.21642082724729686 * t5658 + 4.0 * t7205 - t7744;
    (t7726, t7728, t7730, t7732, t7734, t7735, t7736, t7738, t7744, t7745)
}
