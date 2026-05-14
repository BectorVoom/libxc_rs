//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 941/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk941<F: Float>(t6268: F, t6513: F, t2002: F, t6361: F, t19712: F, t19714: F, t19716: F, t19718: F, t19722: F, t19724: F, t19726: F, t19727: F, t19729: F, t6365: F, t6275: F, t6372: F) -> (F, F, F, F, F) {
    let t19731 = 4.0 / 9.0 * t6268 * t6513;
    let t19733 = 2.0 / 15.0 * t2002 * t6361;
    let t19734 = -t19712 - t19714 - t19716 - t19718 - t19722 - t19724 - t19726 - t19727 + t19729 - t19731 - t19733;
    let t19736 = 2.0 / 15.0 * t2002 * t6365;
    let t19738 = 4.0 / 15.0 * t6275 * t6372;
    (t19731, t19733, t19734, t19736, t19738)
}
