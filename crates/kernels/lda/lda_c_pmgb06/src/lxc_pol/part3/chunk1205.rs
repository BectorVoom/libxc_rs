//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1205/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1205<F: Float>(t12812: F, t12813: F, t12814: F, t12817: F, t12818: F, t12823: F, t12824: F, t12826: F, t12827: F, t12829: F, t12832: F, t12833: F, t12834: F, t12835: F, t12836: F, t12839: F, t12844: F, t12846: F, t12849: F, t12852: F, t12855: F, t12857: F, t9770: F) -> (F, F) {
    let t14392 = t12812 - t12813 - t12814 - t12817 - t12818 - t12823 - t12824 - t12826 + t12827 - t12829 - t12832;
    let t14393 = t12833 + t12834 + t12835 - t9770 + t12836 + t12839 + t12844 - t12846 - t12849 - t12852 + t12855 - t12857;
    (t14392, t14393)
}
