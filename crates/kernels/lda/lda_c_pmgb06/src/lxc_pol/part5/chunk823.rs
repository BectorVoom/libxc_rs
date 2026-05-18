//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 823/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk823<F: Float>(t473: F, t7493: F, t1619: F, t7481: F, t7485: F, t3404: F, t7477: F, t7497: F, t103: F, t7479: F, t7483: F, t7487: F, t7491: F, t7495: F, t7499: F) -> (F, F, F, F, F, F) {
    let t7779 = t473 * t7493;
    let t7782 = t1619 * t7481;
    let t7785 = t1619 * t7485;
    let t7788 = t3404 * t7477;
    let t7791 = t473 * t7497;
    let t7800 = F::new(0.04) * t103 * t7779 + F::new(0.013333333333333334) * t103 * t7782 - F::new(0.006666666666666667) * t103 * t7785 - F::new(0.002962962962962963) * t103 * t7788 - F::new(0.006666666666666667) * t103 * t7791 - F::new(0.03999074074074074) * t7479 - F::new(0.035991666666666665) * t7499 + F::new(0.14396666666666666) * t7483 - F::new(0.07198333333333333) * t7487 - F::new(0.21595) * t7491 + F::new(0.21595) * t7495;
    (t7779, t7782, t7785, t7788, t7791, t7800)
}
