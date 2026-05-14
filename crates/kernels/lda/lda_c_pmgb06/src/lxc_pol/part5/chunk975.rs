//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 975/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk975<F: Float>(t16697: F, t16699: F, t16701: F, t3279: F, t439: F, t7645: F, t2493: F, t5187: F, t2002: F, t6297: F, t1420: F, t7651: F, t2492: F, t4779: F, t16743: F, t1972: F, t6528: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20323 = 4.0 / 45.0 * t16697;
    let t20324 = 4.0 / 45.0 * t16699;
    let t20325 = 8.0 / 45.0 * t16701;
    let t20328 = 2.0 / 9.0 * t439 * t3279 * t7645;
    let t20330 = 2.0 / 15.0 * t5187 * t2493;
    let t20332 = 2.0 / 15.0 * t2002 * t6297;
    let t20334 = 2.0 / 15.0 * t1420 * t7651;
    let t20337 = 2.0 / 15.0 * t439 * t4779 * t2492;
    let t20338 = 2.0 / 15.0 * t16743;
    let t20340 = 2.0 / 5.0 * t1972 * t6528;
    (t20323, t20324, t20325, t20328, t20330, t20332, t20334, t20337, t20338, t20340)
}
