//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk950<F: Float>(t12823: F, t12824: F, t12826: F, t12827: F, t12829: F, t12832: F, t12833: F, t12834: F, t12835: F, t12836: F, t9759: F, t9770: F, t486: F, t5110: F, t1600: F, t1835: F) -> (F, F, F) {
    let t12837 = -t12823 - t12824 - t12826 + t12827 + t9759 - t12829 - t12832 + t12833 + t12834 + t12835 - t9770 + t12836;
    let t12839 = t486 * t5110 / 5.0;
    let t12840 = t1835 * t1600;
    (t12837, t12839, t12840)
}
