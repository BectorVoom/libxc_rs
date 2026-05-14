//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 641/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk641<F: Float>(t2377: F, t3098: F, t332: F, t1619: F, t3092: F, t3404: F, t1: F, t1858: F, t1531: F, t2381: F, t453: F, t36: F, t1438: F, t1525: F, t103: F, t2060: F, t3082: F, t3396: F, t3413: F, t3414: F, t4635: F, t4639: F, t4642: F, t5002: F, t5003: F, t5006: F, t5032: F, t5034: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6145 = t3098 * t2377;
    let t6146 = t6145 * t332;
    let t6147 = t1619 * t6146;
    let t6150 = t3092 * t2377;
    let t6151 = t6150 * t332;
    let t6152 = t3404 * t6151;
    let t6155 = t1858 * t1;
    let t6156 = t1619 * t6155;
    let t6159 = t1531 * t2381;
    let t6160 = t6159 * t332;
    let t6161 = t453 * t6160;
    let t6162 = t36 * t6161;
    let t6164 = t1438 * t2381;
    let t6165 = t6164 * t332;
    let t6166 = t1525 * t6165;
    let t6167 = t36 * t6166;
    let t6175 = -0.015996296296296297 * t3082 + 0.013333333333333334 * t103 * t6147 - 0.002962962962962963 * t103 * t6152 - 0.008888888888888889 * t2060 * t6156 + 0.07198333333333333 * t6162 - 0.023994444444444443 * t6167 - 0.047988888888888886 * t4639 + t5002 - 0.014814814814814815 * t5003 + 0.017777777777777778 * t5006 - 0.03199259259259259 * t4635 + 0.047988888888888886 * t4642 - t3413 - t3414 - t5032 + t5034 - 0.007407407407407408 * t3396;
    (t6145, t6146, t6147, t6150, t6151, t6152, t6155, t6156, t6160, t6161, t6162, t6165, t6166, t6167, t6175)
}
