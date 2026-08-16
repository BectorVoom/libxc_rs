//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1178/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1178<F: Float>(t2325: F, t784: F, t3965: F, t4501: F, t542: F, t12031: F, t348: F, t12475: F, t34: F, t5147: F, t739: F, t21398: F) -> (F, F, F, F) {
    let t21451 = t2325 * t784;
    let t21455 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3965 * t4501 * t21451 * t542;
    let t21459 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3965 * t12031 * t21451 * t348;
    let t21464 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t12475 * t5147 * t739 * t784 * t34;
    let t21467 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3965 * t4501 * t21398;
    (t21455, t21459, t21464, t21467)
}
