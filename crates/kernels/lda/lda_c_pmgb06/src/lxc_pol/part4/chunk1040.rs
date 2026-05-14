//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1040/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1040<F: Float>(t132: F, t137: F, t153: F, t15338: F, t15371: F, t15409: F, t15449: F, t1992: F, t1993: F, t493: F, t4935: F, t1586: F, t6285: F, t15290: F, t15293: F, t15295: F, t15298: F, t15302: F, t15304: F, t15306: F, t15308: F, t15310: F, t15312: F, t15314: F, t15316: F) -> (F, F, F, F) {
    let t15455 = t132 * t137 * (t15338 + t15371 + t15409 + t15449) * t153 / 30.0;
    let t15459 = 2.0 / 15.0 * t493 * t1992 * t1993 * t4935;
    let t15463 = t493 * t1992 * t6285 * t1586 / 5.0;
    let t15464 = -t15290 - t15293 + t15295 + t15298 + t15302 + t15304 + t15306 + t15308 + t15310 + t15312 + t15314 + t15316 + t15455 + t15459 - t15463;
    (t15455, t15459, t15463, t15464)
}
