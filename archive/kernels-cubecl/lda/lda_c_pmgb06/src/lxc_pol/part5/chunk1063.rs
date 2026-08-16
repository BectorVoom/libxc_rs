//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1063/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1063<F: Float>(t2002: F, t6788: F, t16184: F, t1972: F, t6509: F, t6268: F, t6513: F, t6361: F, t19712: F, t19714: F, t19716: F, t19718: F, t19722: F, t19724: F) -> (F, F, F, F, F, F) {
    let t19726 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t6788;
    let t19727 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t16184;
    let t19729 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1972 * t6509;
    let t19731 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t6268 * t6513;
    let t19733 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2002 * t6361;
    let t19734 = -t19712 - t19714 - t19716 - t19718 - t19722 - t19724 - t19726 - t19727 + t19729 - t19731 - t19733;
    (t19726, t19727, t19729, t19731, t19733, t19734)
}
