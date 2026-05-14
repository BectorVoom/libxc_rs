//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1361/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1361<F: Float>(t133: F, t19551: F, t14681: F, t14698: F, t14701: F, t14704: F, t19529: F, t19536: F, t19656: F, t19658: F, t19678: F, t19679: F, t19680: F, t19681: F, t19682: F, t19685: F, t19688: F, t19696: F) -> (F,) {
    let t19782 = t133 * t19551;
    let t19788 = 10.34553 * t133 * t19536 - 1.724255 * t133 * t19529 + 1.1495033333333333 * t19782 + t19656 - t19658 + t19678 + 13.79404 * t14681 - t19679 - t19680 + t19681 + t19682 - t19685 + t19688 + 9.196026666666667 * t14698 - 6.89702 * t14701 - 3.44851 * t14704 + t19696;
    (t19788,)
}
