//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 677/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk677<F: Float>(t159: F, t285: F, t4137: F, t169: F, t274: F, t2817: F, t301: F, t1112: F, t473: F, t483: F, t485: F, t1131: F, t1586: F) -> (F, F, F, F, F) {
    let t4140 = F::cast_from(0.006715335817467199_f64) * t4137 * t159 * t285;
    let t4144 = F::cast_from(0.9247854820715865_f64) * t169 * t2817 * t274 * t301;
    let t4148 = t473 * t1112;
    let t4150 = t4148 * t483 * t485;
    let t4153 = t1586 * t1131 * t485;
    (t4140, t4144, t4148, t4150, t4153)
}
