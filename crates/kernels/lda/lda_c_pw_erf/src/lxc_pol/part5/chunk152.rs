//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 152/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk152<F: Float>(t391: F, t40: F, t357: F, t85: F, t1: F, t60: F, t119: F, t155: F, t84: F) -> (F, F, F, F) {
    let t392 = t40 * t391;
    let t393 = t357 * t85;
    let t394 = F::cast_from(0.019751789702565206_f64) * t393;
    let t395 = t60 * t1;
    let t397 = t119 * t155 * t84;
    (t392, t394, t395, t397)
}
