//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1092/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1092<F: Float>(t542: F, t806: F, t12765: F, t1278: F, t519: F, t1318: F, t1381: F, t5269: F, t593: F, t811: F, t1390: F, t3787: F) -> (F, F, F) {
    let t12771 = t806 * t542;
    let t12775 = F::new(12.0) / F::new(5.0) * t519 * t12765 * t12771 * t1278;
    let t12780 = F::new(8.0) / F::new(5.0) * t1318 * t5269 * t811 * t593 * t1381;
    let t12781 = t3787 * t1390;
    (t12775, t12780, t12781)
}
