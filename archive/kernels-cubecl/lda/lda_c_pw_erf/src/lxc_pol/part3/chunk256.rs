//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 256/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk256<F: Float>(t127: F, t418: F, t425: F, t426: F, t434: F, t762: F, t764: F, t767: F, t133: F, t447: F) -> (F, F) {
    let t770 = -t418 - t762 - t425 - t426 * t764 / F::cast_from(2.0_f64) - t434 - F::cast_from(1.46904_f64) * t127 * t767;
    let t774 = -t418 - t762 - t447 - F::cast_from(1.724255_f64) * t133 * t764;
    (t770, t774)
}
