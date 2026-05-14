//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1022/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1022<F: Float>(t1901: F, t19762: F, t2010: F, t1420: F, t7551: F, t10148: F, t439: F, t7550: F, t2064: F, t2570: F, t2960: F, t187: F, t7704: F, t1963: F, t6127: F, t17875: F) -> (F, F, F, F, F, F, F) {
    let t21050 = 2.0 / 9.0 * t2010 * t1901 * t19762;
    let t21052 = t1420 * t7551 / 9.0;
    let t21055 = t439 * t10148 * t7550 / 9.0;
    let t21059 = t439 * t2960 * t2570 * t2064 / 9.0;
    let t21061 = t7704 * t187;
    let t21065 = t6127 * t1963 / 15.0;
    let t21066 = t17875 / 15.0;
    (t21050, t21052, t21055, t21059, t21061, t21065, t21066)
}
