//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 657/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk657<F: Float>(t1385: F, t6412: F, t439: F, t1897: F, t6160: F, t1901: F, t6165: F, t6374: F, t6378: F, t6381: F, t6384: F, t6386: F, t6389: F, t6393: F, t6397: F, t6401: F, t6405: F, t6409: F, t6411: F) -> (F, F, F, F, F, F, F) {
    let t6413 = t1385 * t6412;
    let t6415 = t439 * t6413 / 45.0;
    let t6416 = t1897 * t6160;
    let t6418 = 2.0 / 45.0 * t439 * t6416;
    let t6419 = t1901 * t6165;
    let t6421 = t439 * t6419 / 27.0;
    let t6422 = -t6374 + t6378 + t6381 - t6384 - t6386 - t6389 - t6393 + t6397 + t6401 + t6405 + t6409 - t6411 - t6415 - t6418 + t6421;
    (t6413, t6415, t6416, t6418, t6419, t6421, t6422)
}
