//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 228/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk228<F: Float>(t226: F, t611: F, t230: F, t231: F, t498: F, t513: F, t517: F, t527: F, t546: F, t553: F, t567: F, t570: F, t579: F, t597: F, t598: F, t606: F) -> (F, F, F) {
    let t613 = F::new(4.0) / F::new(3.0) * t226 * t611;
    let t615 = F::new(4.0) / F::new(3.0) * t226 * t230;
    let t616 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + F::new(4.0) / F::new(3.0) * t598 * t231 + t606 + t613 + t615;
    (t613, t615, t616)
}
