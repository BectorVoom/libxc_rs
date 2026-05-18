//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1245/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1245<F: Float>(t14654: F, t8896: F, t127: F, t3296: F, t14666: F, t431: F, t5571: F, t5509: F, t925: F, t2061: F, t5512: F, t14646: F, t5592: F) -> (F, F, F, F, F, F) {
    let t14781 = t8896 * t14654;
    let t14783 = t127 * t3296;
    let t14787 = t431 * t5571 * t14666;
    let t14795 = t5509 * t925;
    let t14796 = F::new(2.93808) * t14795;
    let t14797 = t5512 * t2061;
    let t14799 = t5592 * t14646;
    (t14781, t14783, t14787, t14796, t14797, t14799)
}
