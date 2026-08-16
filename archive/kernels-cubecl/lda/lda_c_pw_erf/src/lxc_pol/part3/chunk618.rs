//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 618/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk618<F: Float>(t2967: F, t3589: F, t3587: F, t2973: F, t558: F, t589: F, t190: F, t212: F, t3469: F, t1375: F, t331: F, t1350: F, t50: F) -> (F, F, F, F, F, F, F) {
    let t3590 = t3589 * t2967;
    let t3591 = t3587 * t3590;
    let t3594 = t558 * t2973;
    let t3595 = t589 * t3594;
    let t3600 = F::cast_from(0.02962962962962963_f64) * t190 * t3469 * t212;
    let t3601 = t331 * t1375;
    let t3604 = F::cast_from(1.0_f64) / t1350 / t50;
    (t3590, t3591, t3594, t3595, t3600, t3601, t3604)
}
