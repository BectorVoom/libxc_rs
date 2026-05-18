//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 698/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk698<F: Float>(t548: F, t6215: F, t2120: F, t2131: F, t2504: F, t514: F) -> (F, F, F, F) {
    let t6216 = t548 * t6215;
    let t6217 = F::new(8.0) / F::new(45.0) * t6216;
    let t6219 = F::new(8.0) / F::new(15.0) * t2120 * t2131;
    let t6220 = t514 * t2504;
    (t6216, t6217, t6219, t6220)
}
