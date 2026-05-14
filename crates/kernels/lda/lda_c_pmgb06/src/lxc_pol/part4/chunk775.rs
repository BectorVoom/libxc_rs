//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 775/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk775<F: Float>(t3391: F, t3392: F, t3395: F, t5396: F, t5398: F, t5419: F, t5434: F, t5436: F, t5438: F, t5440: F, t5444: F, t5446: F, t5450: F, t5453: F, t5456: F, t4618: F, t5504: F, t5629: F, t5631: F, t5641: F, t5644: F, t5645: F, t5660: F, t5661: F, t5665: F, t5666: F, t5668: F, t5669: F, t5672: F, t5682: F) -> (F,) {
    let t5685 = t3391 + 16.0 / 3.0 * t3392 + t3395 + t5396 + t5398 + t5419 + t5434 + t5436 + t5438 + t5440 + t5444 - t5446 - t5450 - t5453 - t5456;
    let t5689 = t4618 + t5629 + t5631 + t5641 + t5644 + t5645 + t5660 + t5661 + t5665 + t5666 + t5668 + t5669 + t5672 + t5682 + t5685 + t5504;
    (t5689,)
}
