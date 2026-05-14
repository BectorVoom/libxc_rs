//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 880/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk880<F: Float>(t9093: F, t486: F, t5105: F, t161: F, t489: F, t4953: F, t132: F, t137: F, t2106: F, t3441: F, t9104: F, t9106: F, t11751: F, t11756: F, t11758: F, t11759: F, t11760: F) -> (F, F, F, F, F, F, F) {
    let t11761 = t9093 / 15.0;
    let t11762 = t486 * t5105;
    let t11763 = 2.0 / 15.0 * t11762;
    let t11765 = t161 * t489 * t4953;
    let t11766 = t11765 / 15.0;
    let t11770 = t132 * t137 * t2106 * t3441 / 30.0;
    let t11771 = 2.0 / 45.0 * t9104;
    let t11772 = 2.0 / 135.0 * t9106;
    let t11773 = t11751 - t11756 + t11758 + t11759 + t11760 + t11761 - t11763 - t11766 - t11770 + t11771 + t11772;
    (t11761, t11763, t11766, t11770, t11771, t11772, t11773)
}
