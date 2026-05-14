//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 558/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk558<F: Float>(t1402: F, t2466: F, t186: F, t211: F, t806: F) -> (F, F, F, F) {
    let t2467 = t1402 * t2466;
    let t2468 = t186 * t2467;
    let t2470 = 4.0 / 15.0 * t211 * t2468;
    let t2471 = t806 * t806;
    (t2467, t2468, t2470, t2471)
}
