//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1064<F: Float>(t8189: F, t2329: F, t348: F, t462: F, t39: F, t8327: F) -> (F, F, F, F) {
    let t19987 = F::new(0.5848223397455204) * t8189;
    let t19994 = t2329 * t348;
    let t19997 = t462 * t2329;
    let t20007 = F::new(12.0) * t39 + F::new(24.0) * t8327;
    (t19987, t19994, t19997, t20007)
}
