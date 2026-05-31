//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 576/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk576<F: Float>(t590: F, t933: F, t205: F, t558: F, t191: F, t1350: F, t261: F) -> (F, F, F, F) {
    let t3579 = t933 * t590;
    let t3586 = F::cast_from(1.0_f64) / t205 / t558;
    let t3587 = t191 * t3586;
    let t3589 = F::cast_from(1.0_f64) / t1350 / t261;
    (t3579, t3586, t3587, t3589)
}
