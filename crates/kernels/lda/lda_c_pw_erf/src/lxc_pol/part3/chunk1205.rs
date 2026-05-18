//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1205/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1205<F: Float>(t10567: F, t197: F, t11724: F, t519: F, t3892: F, t473: F, t11729: F, t1446: F, t5257: F, t5261: F, t1313: F, t4748: F, t945: F) -> (F, F, F, F, F) {
    let t14200 = t10567 * t197;
    let t14203 = F::new(352.0) / F::new(243.0) * t519 * t14200 * t11724;
    let t14205 = t473 * t3892 * t197;
    let t14208 = F::new(64.0) / F::new(27.0) * t519 * t14205 * t11729;
    let t14210 = F::new(16.0) / F::new(9.0) * t1446 * t5257;
    let t14212 = F::new(4.0) / F::new(15.0) * t1446 * t5261;
    let t14216 = F::new(4.0) / F::new(15.0) * t519 * t1313 * t4748 * t945;
    (t14203, t14208, t14210, t14212, t14216)
}
