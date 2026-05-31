//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1287/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1287<F: Float>(t4602: F, t6513: F, t1981: F, t5463: F, t6512: F, t1447: F, t6131: F, t1989: F, t5194: F, t2562: F, t607: F, t500: F) -> (F, F, F, F, F) {
    let t16916 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4602 * t6513;
    let t16919 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1981 * t5463 * t6512;
    let t16920 = t1447 * t6131;
    let t16921 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t16920;
    let t16922 = t5194 * t1989;
    let t16923 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t16922;
    let t16924 = t2562 * t607;
    let t16925 = t16924 * t500;
    (t16916, t16919, t16921, t16923, t16925)
}
