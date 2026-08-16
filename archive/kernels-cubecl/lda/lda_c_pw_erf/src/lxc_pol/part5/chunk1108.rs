//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1108/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1108<F: Float>(t1446: F, t7621: F, t20007: F, t519: F, t522: F, t523: F, t7625: F, t2554: F, t5327: F, t15521: F, t15525: F, t15538: F) -> (F, F, F, F, F, F, F) {
    let t20670 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1446 * t7621;
    let t20674 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t519 * t522 * t523 * t20007;
    let t20676 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t1446 * t7625;
    let t20678 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5327 * t2554;
    let t20679 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t15521;
    let t20680 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t15525;
    let t20681 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t15538;
    (t20670, t20674, t20676, t20678, t20679, t20680, t20681)
}
