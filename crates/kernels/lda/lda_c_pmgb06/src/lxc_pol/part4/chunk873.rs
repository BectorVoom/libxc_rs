//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 873/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk873<F: Float>(t6381: F, t6384: F, t6386: F, t6389: F, t6393: F, t6397: F, t6401: F, t6405: F, t6409: F, t6411: F, t6415: F, t6418: F, t6421: F, t6424: F, t6426: F, t4786: F, t4788: F, t4792: F, t4794: F, t4807: F, t4809: F, t4812: F, t4814: F, t4950: F, t4970: F, t5633: F, t5640: F, t6427: F, t6428: F, t6429: F) -> (F, F) {
    let t7188 = t6381 - t6384 - t6386 - t6389 - t6393 + t6397 + t6401 + t6405 + t6409 - t6411 - t6415 - t6418 + t6421 - t6424 - t6426;
    let t7190 = t4786 + t4788 + t4792 + t4794 + t4807 + t4809 + t4812 + t4814 + 8.0 / 3.0 * t5633 + t5640 + t6427 + t6428 - t4950 - t4970 - t6429;
    (t7188, t7190)
}
