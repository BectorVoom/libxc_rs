//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 629/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk629<F: Float>(t265: F, t3704: F, t1446: F, t1450: F, t1507: F, t518: F) -> (F, F, F, F) {
    let t3706 = F::new(8.0) / F::new(405.0) * t265 * t3704;
    let t3707 = t1446 * t1450;
    let t3708 = F::new(16.0) / F::new(45.0) * t3707;
    let t3709 = t1507 * t518;
    (t3706, t3707, t3708, t3709)
}
