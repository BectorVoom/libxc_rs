//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 967/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk967<F: Float>(t12862: F, t12829: F, t12832: F, t12836: F, t12839: F, t12842: F, t12844: F, t12846: F, t12848: F, t12853: F, t12855: F, t12857: F, t12860: F, t10313: F, t1967: F, t197: F, t519: F) -> (F, F, F) {
    let t12863 = 16.0 / 135.0 * t12862;
    let t12864 = -t12829 + t12832 - t12836 + t12839 + t12842 + t12844 - t12846 - t12848 - t12853 - t12855 - t12857 - t12860 + t12863;
    let t12869 = t519 * t10313 * t197 * t1967;
    (t12863, t12864, t12869)
}
