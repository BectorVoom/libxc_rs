//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 625/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk625<F: Float>(t213: F, t3667: F, t1403: F, t593: F, t186: F, t211: F, t528: F) -> (F, F, F, F, F, F) {
    let t3668 = t213 * t3667;
    let t3669 = t1403 * t593;
    let t3670 = t3668 * t3669;
    let t3671 = t186 * t3670;
    let t3673 = F::new(4.0) / F::new(5.0) * t211 * t3671;
    let t3674 = t528 * t528;
    let t3675 = F::new(1.0) / t3674;
    (t3669, t3670, t3671, t3673, t3674, t3675)
}
