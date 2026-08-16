//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1249/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1249<F: Float>(t14584: F, t426: F, t325: F, t431: F, t5565: F, t1686: F, t1856: F, t933: F, t14587: F, t127: F, t14632: F, t14684: F, t14686: F, t14689: F, t14692: F, t14695: F, t14719: F, t436: F) -> F {
    let t14843 = t426 * t14584;
    let t14844 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14843;
    let t14846 = t431 * t5565 * t325;
    let t14849 = t1686 * t1856 * t933;
    let t14850 = F::cast_from(1.46904_f64) * t14849;
    let t14851 = t426 * t14587;
    let t14853 = -t14684 - t14686 + t14689 + t14692 - F::cast_from(1.46904_f64) * t127 * t436 * t14632 - t14695 - t14844 + F::cast_from(2.20356_f64) * t14846 - t14850 + t14851 / F::cast_from(2.0_f64) + t14719;
    t14853
}
