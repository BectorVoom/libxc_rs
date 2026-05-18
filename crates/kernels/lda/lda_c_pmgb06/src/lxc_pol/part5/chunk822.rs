//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 822/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk822<F: Float>(t6627: F, t6710: F, t473: F, t7489: F, t103: F, t3413: F, t3414: F, t4635: F, t5003: F, t6205: F, t6207: F, t6209: F, t6211: F, t6213: F, t6215: F) -> (F, F, F, F) {
    let t7765 = F::new(2.0) / F::new(15.0) * t6627;
    let t7766 = F::new(2.0) / F::new(15.0) * t6710;
    let t7775 = t473 * t7489;
    let t7778 = F::new(0.023994444444444443) * t6205 - F::new(0.07198333333333333) * t6207 + F::new(0.035991666666666665) * t6209 - F::new(0.02666666666666667) * t6211 + F::new(0.013333333333333334) * t6213 + F::new(0.0044444444444444444) * t6215 - t3413 - t3414 - F::new(0.022222222222222223) * t5003 - F::new(0.047988888888888886) * t4635 - F::new(0.04) * t103 * t7775;
    (t7765, t7766, t7775, t7778)
}
