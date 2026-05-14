//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1046/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1046<F: Float>(t11729: F, t14205: F, t519: F, t1446: F, t5257: F, t5261: F, t1313: F, t4748: F, t945: F, t1995: F, t2961: F, t5222: F, t1245: F, t2098: F, t3402: F, t940: F) -> (F, F, F, F, F, F, F) {
    let t14208 = 64.0 / 27.0 * t519 * t14205 * t11729;
    let t14210 = 16.0 / 9.0 * t1446 * t5257;
    let t14212 = 4.0 / 15.0 * t1446 * t5261;
    let t14216 = 4.0 / 15.0 * t519 * t1313 * t4748 * t945;
    let t14220 = 4.0 / 45.0 * t519 * t1313 * t1995 * t2961;
    let t14222 = 4.0 / 9.0 * t1446 * t5222;
    let t14227 = 4.0 / 9.0 * t519 * t3402 * t2098 * t1245 * t940;
    (t14208, t14210, t14212, t14216, t14220, t14222, t14227)
}
