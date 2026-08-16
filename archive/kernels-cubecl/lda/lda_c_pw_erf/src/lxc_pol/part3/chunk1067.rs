//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1067/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1067<F: Float>(t12498: F, t184: F, t202: F, t4701: F, t551: F, t9540: F, t9590: F, t12474: F, t12480: F, t12482: F, t12485: F, t12488: F, t12491: F, t12495: F, t12497: F) -> (F, F, F, F, F) {
    let t12499 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t12498;
    let t12501 = t202 * t4701 * t184;
    let t12503 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t12501 * t551;
    let t12504 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9540;
    let t12505 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9590;
    let t12506 = -t12474 - t12480 - t12482 - t12485 - t12488 - t12491 - t12495 - t12497 + t12499 + t12503 + t12504 + t12505;
    (t12499, t12503, t12504, t12505, t12506)
}
