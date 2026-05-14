//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 568/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk568<F: Float>(t256: F, t3933: F, t635: F, t646: F, t645: F, t1432: F, t639: F, t1423: F, t1427: F, t695: F, t1198: F, t1426: F, t458: F, t108: F, t492: F, t267: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3935 = t3933 * t256 / 3.0;
    let t3936 = t635 * t646;
    let t3938 = 0.013506172839506173 * t645 * t3936;
    let t3949 = t639 * t1432;
    let t3950 = t3949 * t256;
    let t3951 = t1423 * t1427;
    let t3959 = 0.06649088888888889 * t695 * t646;
    let t3960 = t1198 * t646;
    let t3963 = 0.09973633333333333 * t458 * t1426;
    let t3964 = t492 * t108;
    let t3965 = t3964 * t267;
    (t3935, t3936, t3938, t3949, t3950, t3951, t3959, t3960, t3963, t3964, t3965)
}
