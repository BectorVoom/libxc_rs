//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 688/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk688<F: Float>(t2061: F, t4913: F, t1619: F, t4650: F, t4663: F, t473: F, t4659: F, t3404: F, t4645: F, t4655: F, t4672: F, t4668: F, t2057: F, t405: F, t2054: F, t103: F, t2060: F, t3396: F, t3398: F, t3400: F, t3413: F, t3414: F, t4635: F, t4642: F) -> (F, F, F, F, F, F, F, F) {
    let t5006 = t4913 * t2061;
    let t5010 = t1619 * t4650;
    let t5013 = t473 * t4663;
    let t5016 = t1619 * t4659;
    let t5019 = t3404 * t4645;
    let t5022 = t1619 * t4655;
    let t5025 = t473 * t4672;
    let t5028 = t473 * t4668;
    let t5032 = 0.017777777777777778 * t405 * t2057;
    let t5034 = 0.002962962962962963 * t405 * t2054;
    let t5038 = 0.057777777777777775 * t5006 - 0.015996296296296297 * t4635 + 0.2639388888888889 * t4642 - t3413 - t3414 + 0.013333333333333334 * t103 * t5010 - 0.04 * t103 * t5013 - 0.0022222222222222222 * t103 * t5016 - 0.002962962962962963 * t103 * t5019 - 0.008888888888888889 * t2060 * t5022 + 0.013333333333333334 * t103 * t5025 + 0.05333333333333334 * t2060 * t5028 - t5032 + t5034 - 0.014814814814814815 * t3396 + 0.0044444444444444444 * t3398 + 0.0014814814814814814 * t3400;
    (t5010, t5013, t5016, t5019, t5022, t5025, t5028, t5038)
}
