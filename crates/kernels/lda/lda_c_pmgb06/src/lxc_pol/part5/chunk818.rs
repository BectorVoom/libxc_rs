//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 818/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk818<F: Float>(t2606: F, t802: F, t2599: F, t5108: F, t166: F, t161: F, t2601: F, t831: F, t2563: F, t853: F, t2654: F, t6734: F, t822: F) -> (F, F, F, F, F, F, F, F) {
    let t7723 = t802 * t2606 / F::new(5.0);
    let t7725 = t5108 * t2599;
    let t7726 = t166 * t7725;
    let t7728 = t161 * t7726 / F::new(5.0);
    let t7730 = t831 * t2601 / F::new(5.0);
    let t7732 = t2563 * t853 / F::new(10.0);
    let t7734 = t831 * t2654 / F::new(5.0);
    let t7735 = t6734 * t822;
    (t7723, t7725, t7726, t7728, t7730, t7732, t7734, t7735)
}
