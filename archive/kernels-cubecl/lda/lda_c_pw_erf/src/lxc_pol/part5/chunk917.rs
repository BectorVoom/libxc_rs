//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 917/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk917<F: Float>(t1159: F, t603: F, t1634: F, t695: F, t2070: F, t493: F, t495: F, t4039: F, t511: F, t220: F, t4567: F, t211: F) -> (F, F, F, F, F) {
    let t10414 = t1159 * t603;
    let t10417 = F::cast_from(0.004413481481481482_f64) * t695 * t1634;
    let t10419 = t493 * t2070 * t495;
    let t10427 = t511 * t4039;
    let t10436 = t4567 * t220;
    let t10438 = F::cast_from(112.0_f64) / F::cast_from(1215.0_f64) * t211 * t10436;
    (t10414, t10417, t10419, t10427, t10438)
}
