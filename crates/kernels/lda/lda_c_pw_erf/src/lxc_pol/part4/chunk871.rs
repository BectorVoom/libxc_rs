//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 871/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk871<F: Float>(t589: F, t6413: F, t6446: F, t6361: F, t1371: F, t6366: F, t6379: F, t3587: F, t6384: F, t6388: F, t2061: F, t25: F, t4657: F, t4663: F, t5022: F, t5024: F, t6641: F, t6644: F, t6647: F, t6652: F, t6655: F, t6660: F) -> (F, F, F, F, F, F, F, F) {
    let t6812 = t589 * t6413;
    let t6815 = t589 * t6446;
    let t6818 = t589 * t6361;
    let t6821 = t1371 * t6366;
    let t6824 = t1371 * t6379;
    let t6827 = t3587 * t6384;
    let t6830 = t1371 * t6388;
    let t6842 = -0.023994444444444443 * t6660 - 0.04 * t25 * t6812 - 0.05333333333333334 * t2061 * t6815 + 0.013333333333333334 * t25 * t6818 - 0.0022222222222222222 * t25 * t6821 + 0.013333333333333334 * t25 * t6824 - 0.002962962962962963 * t25 * t6827 + 0.008888888888888889 * t2061 * t6830 + 0.14396666666666666 * t6644 - 0.03999074074074074 * t6641 + 0.09597777777777777 * t6647 - 0.21595 * t6652 - 0.2879333333333333 * t6655 - 0.017777777777777778 * t5022 - 0.014814814814814815 * t5024 - 0.03199259259259259 * t4657 - 0.047988888888888886 * t4663;
    (t6812, t6815, t6818, t6821, t6824, t6827, t6830, t6842)
}
