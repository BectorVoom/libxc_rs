//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1189/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1189<F: Float>(t12765: F, t21583: F, t519: F, t542: F, t1325: F, t2497: F, t5289: F, t784: F, t1318: F, t2478: F, t5269: F, t593: F, t833: F) -> (F, F, F) {
    let t21591 = F::cast_from(12.0_f64) / F::cast_from(5.0_f64) * t519 * t12765 * t21583 * t542;
    let t21596 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1325 * t5289 * t2497 * t784 * t542;
    let t21601 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1318 * t5269 * t2478 * t833 * t593;
    (t21591, t21596, t21601)
}
