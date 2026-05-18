//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 578/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk578<F: Float>(t560: F, t925: F, t1484: F, t56: F, t174: F, t205: F, t3540: F, t1518: F, t550: F, t548: F, t594: F, t211: F) -> (F, F, F, F, F, F, F, F) {
    let t3627 = t925 * t560;
    let t3633 = t56 * t1484;
    let t3638 = t174 * t3540 * t205;
    let t3639 = F::new(0.11197407407407407) * t3638;
    let t3660 = t1518 * t550;
    let t3661 = t548 * t3660;
    let t3663 = t1518 * t594;
    let t3664 = t211 * t3663;
    (t3627, t3633, t3638, t3639, t3660, t3661, t3663, t3664)
}
