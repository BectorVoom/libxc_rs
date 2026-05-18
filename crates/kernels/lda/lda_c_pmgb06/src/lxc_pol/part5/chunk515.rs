//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 515/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk515<F: Float>(t137: F, t2605: F, t132: F, t1576: F, t2541: F, t2545: F, t525: F, t2549: F, t103: F, t1563: F, t1571: F, t1818: F, t2077: F, t2543: F, t2547: F, t2551: F) -> (F, F, F, F, F, F) {
    let t2606 = t137 * t2605;
    let t2608 = t132 * t2606 / F::new(15.0);
    let t2614 = t1576 * t2541;
    let t2617 = t525 * t2545;
    let t2620 = t525 * t2549;
    let t2623 = t1563 + F::new(0.023994444444444443) * t1818 - F::new(0.023994444444444443) * t2543 + F::new(0.07198333333333333) * t2547 - F::new(0.035991666666666665) * t2551 + t1571 + F::new(0.008888888888888889) * t2077 - F::new(0.0022222222222222222) * t103 * t2614 + F::new(0.013333333333333334) * t103 * t2617 - F::new(0.006666666666666667) * t103 * t2620;
    (t2606, t2608, t2614, t2617, t2620, t2623)
}
