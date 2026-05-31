//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1110/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1110<F: Float>(t2492: F, t439: F, t4779: F, t16743: F, t1972: F, t6528: F, t6254: F, t6550: F, t6258: F, t20323: F, t20324: F, t20325: F, t20328: F, t20330: F, t20332: F, t20334: F) -> (F, F, F, F, F, F) {
    let t20337 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t4779 * t2492;
    let t20338 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16743;
    let t20340 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t1972 * t6528;
    let t20343 = F::cast_from(3.0_f64) / F::cast_from(5.0_f64) * t439 * t6550 * t6254;
    let t20346 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t439 * t6550 * t6258;
    let t20347 = t20323 + t20324 + t20325 - t20328 - t20330 - t20332 - t20334 - t20337 + t20338 + t20340 - t20343 + t20346;
    (t20337, t20338, t20340, t20343, t20346, t20347)
}
