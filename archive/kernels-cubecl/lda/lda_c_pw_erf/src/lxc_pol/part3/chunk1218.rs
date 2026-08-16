//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1218/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1218<F: Float>(t4713: F, t473: F, t483: F, t485: F, t1131: F, t5474: F, t1910: F, t2910: F, t1124: F, t1904: F, t5470: F, t1191: F, t780: F) -> (F, F, F, F, F, F) {
    let t14382 = t473 * t4713 * t483 * t485;
    let t14385 = t5474 * t1131 * t485;
    let t14386 = F::cast_from(0.01185233419734569_f64) * t14385;
    let t14388 = t1910 * t2910 * t485;
    let t14392 = t1124 * t1904 * t483 * t485;
    let t14393 = F::cast_from(0.01975389032890948_f64) * t14392;
    let t14395 = t5470 * t1131 * t485;
    let t14397 = t1191 * t780;
    (t14382, t14386, t14388, t14393, t14395, t14397)
}
