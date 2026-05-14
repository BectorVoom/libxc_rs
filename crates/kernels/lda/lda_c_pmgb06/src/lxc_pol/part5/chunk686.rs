//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 686/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk686<F: Float>(t2617: F, t405: F, t2620: F, t2614: F, t525: F, t6827: F, t1576: F, t6503: F, t3358: F, t6508: F, t6512: F, t6402: F, t6406: F, t6760: F, t6765: F, t103: F, t2060: F, t3368: F, t3380: F, t6800: F, t6811: F, t6819: F, t6829: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6873 = t405 * t2617;
    let t6875 = t405 * t2620;
    let t6877 = t405 * t2614;
    let t6879 = t525 * t6827;
    let t6882 = t1576 * t6503;
    let t6885 = t3358 * t6508;
    let t6888 = t1576 * t6512;
    let t6891 = t525 * t6402;
    let t6894 = t525 * t6406;
    let t6897 = t525 * t6760;
    let t6900 = t1576 * t6765;
    let t6903 = -t3368 - t3380 + 0.007998148148148148 * t6800 - 0.023994444444444443 * t6811 + 0.011997222222222222 * t6819 - 0.035991666666666665 * t6829 - 0.008888888888888889 * t6873 + 0.0044444444444444444 * t6875 + 0.0014814814814814814 * t6877 - 0.006666666666666667 * t103 * t6879 + 0.013333333333333334 * t103 * t6882 - 0.002962962962962963 * t103 * t6885 + 0.008888888888888889 * t2060 * t6888 - 0.04 * t103 * t6891 - 0.05333333333333334 * t2060 * t6894 + 0.013333333333333334 * t103 * t6897 - 0.0022222222222222222 * t103 * t6900;
    (t6873, t6875, t6877, t6879, t6882, t6885, t6888, t6891, t6894, t6897, t6900, t6903)
}
