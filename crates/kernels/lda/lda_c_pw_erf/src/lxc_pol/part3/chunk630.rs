//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 630/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk630<F: Float>(t3709: F, t525: F, t1336: F, t1472: F, t219: F, t3604: F, t2967: F, t1485: F, t571: F, t1341: F, t1446: F, t267: F, t3571: F, t3573: F, t3575: F, t3578: F, t3659: F, t3662: F, t3665: F, t3673: F, t3681: F, t3682: F, t3684: F, t3701: F, t3706: F, t3708: F) -> (F, F, F, F, F, F, F) {
    let t3711 = F::new(4.0) / F::new(15.0) * t3709 * t525;
    let t3713 = F::new(8.0) / F::new(15.0) * t1472 * t1336;
    let t3714 = t219 * t3604;
    let t3715 = t3714 * t2967;
    let t3716 = t1485 * t3715;
    let t3718 = F::new(8.0) / F::new(9.0) * t571 * t3716;
    let t3720 = F::new(8.0) / F::new(15.0) * t1446 * t1341;
    let t3721 = -t3571 + t3573 - t3575 - t3578 - t3659 - t3662 + t3665 - t3673 - t3681 + F::new(2.0) / F::new(45.0) * t3682 - F::new(2.0) / F::new(15.0) * t3684 - t3701 * t267 / F::new(15.0) - t3706 + t3708 + t3711 - t3713 - t3718 - t3720;
    (t3711, t3713, t3715, t3716, t3718, t3720, t3721)
}
