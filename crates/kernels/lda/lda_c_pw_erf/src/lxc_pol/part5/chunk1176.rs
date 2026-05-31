//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1176/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1176<F: Float>(t9947: F, t184: F, t2441: F, t494: F, t786: F, t2067: F, t2425: F, t784: F, t793: F, t2131: F, t493: F, t514: F, t7798: F) -> (F, F, F, F, F) {
    let t21432 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t9947;
    let t21436 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t494 * t2441 * t184 * t786;
    let t21438 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t2425 * t2067;
    let t21440 = t784 * t793 * t184;
    let t21442 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t21440 * t2131;
    let t21444 = t493 * t514 * t7798;
    (t21432, t21436, t21438, t21442, t21444)
}
