//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1044/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1044<F: Float>(t1308: F, t2021: F, t2973: F, t571: F, t1472: F, t4820: F, t1351: F, t2065: F, t3832: F, t951: F, t10379: F, t2967: F, t3589: F, t833: F) -> (F, F, F, F) {
    let t12227 = F::new(4.0) / F::new(45.0) * t571 * t1308 * t2021 * t2973;
    let t12229 = F::new(4.0) / F::new(9.0) * t1472 * t4820;
    let t12234 = F::new(4.0) / F::new(9.0) * t571 * t3832 * t2065 * t1351 * t951;
    let t12239 = F::new(32.0) / F::new(81.0) * t571 * t10379 * t833 * t3589 * t2967;
    (t12227, t12229, t12234, t12239)
}
