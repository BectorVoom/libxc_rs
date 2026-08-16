//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1169/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1169<F: Float>(t21359: F, t12685: F, t12709: F, t21336: F, t21338: F, t21342: F, t21344: F, t21346: F, t21349: F, t21351: F, t21353: F, t21355: F, t21358: F) -> (F, F) {
    let t21360 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t21359;
    let t21361 = -t12685 - t12709 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t21336 + t21338 + t21342 - t21344 + t21346 + t21349 + t21351 + t21353 - t21355 - t21358 - t21360;
    (t21360, t21361)
}
