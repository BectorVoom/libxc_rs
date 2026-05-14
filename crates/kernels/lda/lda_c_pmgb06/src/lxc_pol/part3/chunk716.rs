//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 716/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk716<F: Float>(t1897: F, t4672: F, t439: F, t1901: F, t4650: F, t4668: F, t2010: F, t1420: F, t1902: F, t153: F, t3279: F, t1859: F, t4659: F, t3260: F, t4645: F, t4655: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5242 = t1897 * t4672;
    let t5244 = 2.0 / 45.0 * t439 * t5242;
    let t5245 = t1901 * t4650;
    let t5247 = 2.0 / 9.0 * t439 * t5245;
    let t5248 = t1897 * t4668;
    let t5250 = 8.0 / 45.0 * t2010 * t5248;
    let t5252 = 2.0 / 27.0 * t1420 * t1902;
    let t5253 = t3279 * t153;
    let t5254 = t5253 * t1859;
    let t5256 = 2.0 / 27.0 * t439 * t5254;
    let t5257 = t1901 * t4659;
    let t5259 = t439 * t5257 / 27.0;
    let t5260 = t3260 * t153;
    let t5261 = t5260 * t4645;
    let t5263 = 8.0 / 81.0 * t439 * t5261;
    let t5264 = t1901 * t4655;
    (t5242, t5244, t5245, t5247, t5248, t5250, t5252, t5253, t5254, t5256, t5257, t5259, t5260, t5261, t5263, t5264)
}
