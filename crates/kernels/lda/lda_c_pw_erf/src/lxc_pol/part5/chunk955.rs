//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 955/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk955<F: Float>(t108: F, t2103: F, t267: F, t219: F, t4048: F, t473: F, t10467: F, t2030: F, t519: F, t518: F, t5214: F, t3663: F, t822: F) -> (F, F, F, F, F) {
    let t12143 = t2103 * t108 * t267;
    let t12158 = t473 * t4048 * t219;
    let t12196 = t519 * t10467 * t2030;
    let t12197 = F::new(8.0) / F::new(135.0) * t12196;
    let t12299 = t5214 * t518;
    let t12309 = t822 * t3663;
    (t12143, t12158, t12197, t12299, t12309)
}
