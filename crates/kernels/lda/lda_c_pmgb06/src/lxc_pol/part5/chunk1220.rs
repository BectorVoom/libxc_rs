//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1220/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1220<F: Float>(t11944: F, t19679: F, t19680: F, t19681: F, t19682: F, t19683: F, t19685: F, t19687: F, t19689: F, t19693: F, t9408: F, t11964: F, t12038: F, t19694: F, t19696: F, t19697: F, t19698: F, t9410: F, t9412: F, t9417: F, t9422: F, t9426: F, t9429: F) -> (F, F) {
    let t21922 = t19679 + t19680 - t19681 + t19682 - F::new(0.19947266666666666) * t11944 + t19683 + t19685 - t19687 - t19689 - t19693 - t9408;
    let t21925 = t9410 + t9412 - t9417 + F::new(2.0) / F::new(3.0) * t9422 + t11964 + F::new(8.0) / F::new(81.0) * t9426 + t9429 - t12038 - t19694 - t19696 + t19697 + t19698;
    (t21922, t21925)
}
