//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1382/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1382<F: Float>(t15304: F, t15306: F, t15308: F, t15310: F, t15312: F, t15314: F, t15316: F, t15455: F, t15459: F, t15463: F, t15468: F, t15469: F, t15470: F, t15471: F, t15473: F) -> F {
    let t18163 = t15304 + t15306 + t15308 + t15310 + t15312 + t15314 + t15316 + t15455 + t15459 - t15463 - t15468 + t15469 - t15470 - t15471 - t15473;
    t18163
}
