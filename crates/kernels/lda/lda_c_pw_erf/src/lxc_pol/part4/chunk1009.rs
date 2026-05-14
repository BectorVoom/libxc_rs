//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1009/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1009<F: Float>(t542: F, t806: F, t1390: F, t3787: F, t1325: F, t5291: F, t5040: F, t518: F, t5044: F, t10463: F, t2026: F, t2022: F, t571: F, t9313: F, t3863: F, t5306: F) -> (F, F, F, F, F, F, F, F) {
    let t12771 = t806 * t542;
    let t12781 = t3787 * t1390;
    let t12783 = t1325 * t12781 * t5291;
    let t12794 = t5040 * t518;
    let t12797 = t5044 * t518;
    let t12809 = t1325 * t10463 * t2026;
    let t12814 = t571 * t9313 * t2022;
    let t12817 = t571 * t3863 * t5306;
    (t12771, t12781, t12783, t12794, t12797, t12809, t12814, t12817)
}
