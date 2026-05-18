//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1163/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1163<F: Float>(t6568: F, t795: F, t2120: F, t6592: F, t16971: F, t2505: F, t6209: F, t2104: F, t7838: F, t16935: F, t16949: F, t16952: F) -> (F, F, F, F, F, F, F, F) {
    let t21309 = F::new(2.0) / F::new(5.0) * t795 * t6568;
    let t21311 = F::new(4.0) / F::new(5.0) * t2120 * t6592;
    let t21313 = F::new(4.0) / F::new(5.0) * t16971 * t2505;
    let t21315 = F::new(4.0) / F::new(5.0) * t6209 * t6592;
    let t21317 = F::new(4.0) / F::new(15.0) * t2104 * t7838;
    let t21318 = F::new(8.0) / F::new(27.0) * t16935;
    let t21319 = F::new(16.0) / F::new(45.0) * t16949;
    let t21320 = F::new(8.0) / F::new(45.0) * t16952;
    (t21309, t21311, t21313, t21315, t21317, t21318, t21319, t21320)
}
