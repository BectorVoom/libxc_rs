//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 508/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk508<F: Float>(t2526: F, t582: F, t186: F, t211: F, t2191: F, t833: F, t1466: F) -> (F, F, F, F, F) {
    let t2527 = t582 * t2526;
    let t2528 = t186 * t2527;
    let t2530 = F::new(2.0) / F::new(15.0) * t211 * t2528;
    let t2531 = t2191 * t833;
    let t2532 = t1466 * t2531;
    (t2527, t2528, t2530, t2531, t2532)
}
