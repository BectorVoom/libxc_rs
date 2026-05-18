//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1242/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1242<F: Float>(t1318: F, t1466: F, t22318: F, t549: F, t571: F, t593: F, t7513: F, t9237: F, t1325: F, t3787: F, t7588: F, t18608: F, t826: F) -> (F, F, F, F) {
    let t22322 = F::new(8.0) / F::new(5.0) * t1318 * t1466 * t22318 * t549;
    let t22327 = F::new(16.0) / F::new(5.0) * t571 * t1466 * t9237 * t7513 * t593;
    let t22329 = t1325 * t3787 * t7588;
    let t22330 = F::new(8.0) / F::new(15.0) * t22329;
    let t22332 = F::new(8.0) / F::new(15.0) * t18608 * t826;
    (t22322, t22327, t22330, t22332)
}
