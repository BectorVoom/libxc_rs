//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 586/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk586<F: Float>(t1869: F, t4641: F, t1: F, t1438: F, t1531: F, t10: F, t15: F, t1959: F, t607: F, t1710: F, t883: F, t1447: F, t1912: F, t1916: F, t1920: F, t1730: F, t871: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4642 = t4641 * t1869;
    let t4654 = t1438 * t1;
    let t4667 = t1531 * t1;
    let t4687 = t10 * t1;
    let t4700 = t15 * t1;
    let t4717 = 4.0 / 45.0 * t1959 * t607;
    let t4718 = t883 * t1710;
    let t4721 = 4.0 / 135.0 * t1447 * t1912;
    let t4723 = 8.0 / 135.0 * t1447 * t1916;
    let t4725 = 4.0 / 81.0 * t1447 * t1920;
    let t4740 = t871 * t1730;
    (t4642, t4654, t4667, t4687, t4700, t4717, t4718, t4721, t4723, t4725, t4740)
}
