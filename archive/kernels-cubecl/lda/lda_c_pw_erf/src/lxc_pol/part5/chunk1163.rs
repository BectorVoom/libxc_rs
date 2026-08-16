//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1163/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1163<F: Float>(t6568: F, t795: F, t2120: F, t6592: F, t16971: F, t2505: F, t6209: F, t2104: F, t7838: F, t16935: F, t16949: F, t16952: F) -> (F, F, F, F, F, F, F, F) {
    let t21309 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t795 * t6568;
    let t21311 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2120 * t6592;
    let t21313 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t16971 * t2505;
    let t21315 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t6209 * t6592;
    let t21317 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2104 * t7838;
    let t21318 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16935;
    let t21319 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t16949;
    let t21320 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16952;
    (t21309, t21311, t21313, t21315, t21317, t21318, t21319, t21320)
}
