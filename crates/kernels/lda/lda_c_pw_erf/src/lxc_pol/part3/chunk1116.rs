//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1116/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1116<F: Float>(t13054: F, t1446: F, t4750: F, t1472: F, t5286: F, t1401: F, t2151: F, t1403: F, t1954: F, t571: F, t519: F, t5221: F, t9723: F) -> (F, F, F, F, F) {
    let t13055 = F::new(8.0) / F::new(45.0) * t13054;
    let t13057 = F::new(8.0) / F::new(15.0) * t1446 * t4750;
    let t13059 = F::new(8.0) / F::new(15.0) * t1472 * t5286;
    let t13060 = t2151 * t1401;
    let t13064 = F::new(16.0) / F::new(15.0) * t571 * t13060 * t1954 * t1403;
    let t13066 = t519 * t9723 * t5221;
    (t13055, t13057, t13059, t13064, t13066)
}
