//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk989<F: Float>(t9104: F, t9106: F, t11751: F, t11756: F, t11758: F, t11759: F, t11760: F, t11761: F, t11763: F, t11766: F, t11770: F, t9108: F) -> (F, F, F, F) {
    let t11771 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t9104;
    let t11772 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t9106;
    let t11773 = t11751 - t11756 + t11758 + t11759 + t11760 + t11761 - t11763 - t11766 - t11770 + t11771 + t11772;
    let t11774 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9108;
    (t11771, t11772, t11773, t11774)
}
