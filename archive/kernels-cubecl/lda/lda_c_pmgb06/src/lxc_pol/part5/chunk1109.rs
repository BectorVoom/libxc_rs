//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1109/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1109<F: Float>(t16697: F, t16699: F, t16701: F, t3279: F, t439: F, t7645: F, t2493: F, t5187: F, t2002: F, t6297: F, t1420: F, t7651: F) -> (F, F, F, F, F, F, F) {
    let t20323 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16697;
    let t20324 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16699;
    let t20325 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16701;
    let t20328 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t439 * t3279 * t7645;
    let t20330 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5187 * t2493;
    let t20332 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t6297;
    let t20334 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t7651;
    (t20323, t20324, t20325, t20328, t20330, t20332, t20334)
}
