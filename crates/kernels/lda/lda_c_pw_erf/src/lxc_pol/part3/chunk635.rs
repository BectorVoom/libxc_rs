//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 635/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk635<F: Float>(t3745: F, t525: F, t1335: F, t1475: F, t571: F, t1486: F, t2967: F, t574: F, t1381: F, t1401: F, t593: F, t1466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3747 = F::new(8.0) / F::new(15.0) * t3745 * t525;
    let t3748 = t1475 * t1335;
    let t3749 = t571 * t3748;
    let t3750 = F::new(16.0) / F::new(45.0) * t3749;
    let t3751 = t1486 * t2967;
    let t3752 = t574 * t3751;
    let t3754 = F::new(8.0) / F::new(15.0) * t571 * t3752;
    let t3756 = t1401 * t1381 * t593;
    let t3757 = t1466 * t3756;
    (t3747, t3748, t3749, t3750, t3751, t3752, t3754, t3756, t3757)
}
