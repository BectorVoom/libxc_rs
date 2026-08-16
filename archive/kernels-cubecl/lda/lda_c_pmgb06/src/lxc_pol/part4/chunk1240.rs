//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1240/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1240<F: Float>(t131: F, t16332: F, t178: F, t44: F, t513: F, t6688: F, t12447: F, t12449: F, t2002: F, t4780: F, t224: F, t6704: F) -> (F, F, F, F, F, F) {
    let t16336 = t16332 * t44 * t131 * t178 / F::cast_from(30.0_f64);
    let t16338 = t6688 * t513 / F::cast_from(15.0_f64);
    let t16339 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12447;
    let t16340 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12449;
    let t16342 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t2002 * t4780;
    let t16343 = t6704 * t224;
    (t16336, t16338, t16339, t16340, t16342, t16343)
}
