//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1073/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1073<F: Float>(t2948: F, t439: F, t6412: F, t1385: F, t477: F, t6217: F, t1629: F, t2578: F, t2002: F, t5238: F, t1447: F, t6399: F, t6403: F, t6504: F, t5499: F, t6407: F) -> (F, F, F, F, F, F, F, F) {
    let t16040 = 2.0 / 45.0 * t439 * t2948 * t6412;
    let t16044 = 2.0 / 45.0 * t439 * t1385 * t6217 * t477;
    let t16048 = t439 * t1385 * t2578 * t1629 / 45.0;
    let t16050 = 4.0 / 45.0 * t2002 * t5238;
    let t16051 = t1447 * t6399;
    let t16052 = 8.0 / 135.0 * t16051;
    let t16053 = t1447 * t6403;
    let t16054 = 8.0 / 45.0 * t16053;
    let t16055 = t1447 * t6504;
    let t16056 = 8.0 / 27.0 * t16055;
    let t16057 = t5499 * t6407;
    (t16040, t16044, t16048, t16050, t16052, t16054, t16056, t16057)
}
