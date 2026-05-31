//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 433/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk433<F: Float>(t1680: F, t209: F, t1166: F, t205: F, t208: F, t398: F, t579: F, t213: F, t573: F, t97: F, t588: F, t947: F, t955: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1682 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t209 * t1680;
    let t1683 = t1166 * t205;
    let t1684 = t1683 * t208;
    let t1687 = t398 * t579;
    let t1688 = t1687 * t208;
    let t1689 = t1688 * t213;
    let t1691 = t573 * t97;
    let t1692 = t1691 * t588;
    let t1696 = -F::cast_from(0.043111111111111114_f64) * t947 + F::cast_from(0.18777777777777777_f64) * t955;
    (t1682, t1683, t1684, t1687, t1688, t1689, t1691, t1692, t1696)
}
