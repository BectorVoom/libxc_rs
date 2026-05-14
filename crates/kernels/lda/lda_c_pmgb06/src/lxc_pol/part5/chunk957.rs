//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 957/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk957<F: Float>(t1919: F, t19389: F, t1981: F, t1385: F, t15935: F, t439: F, t760: F, t1: F, t2010: F, t6773: F, t10699: F, t20009: F, t20012: F, t20014: F, t20017: F, t20021: F, t20025: F, t20028: F) -> (F, F, F, F) {
    let t20031 = 4.0 / 3.0 * t1981 * t1919 * t19389;
    let t20035 = t439 * t1385 * t15935 * t760 / 15.0;
    let t20039 = 2.0 / 15.0 * t2010 * t1385 * t6773 * t1;
    let t20040 = 4.0 * t10699 - t20009 - t20012 - t20014 - t20017 - t20021 + t20025 - t20028 + t20031 - t20035 - t20039;
    (t20031, t20035, t20039, t20040)
}
