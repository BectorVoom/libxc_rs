//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1185/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1185<F: Float>(t2409: F, t9883: F, t3965: F, t14469: F, t14657: F, t3825: F, t3990: F, t3991: F, t3989: F, t3855: F, t14116: F, t3835: F) -> (F, F, F, F, F, F, F, F) {
    let t15345 = t2409 * t9883;
    let t15346 = t3965 * t15345;
    let t15348 = t14657 * t14469;
    let t15357 = t3990 * t3991 * t3825;
    let t15358 = t3989 * t15357;
    let t15366 = t3990 * t3991 * t3855;
    let t15367 = t3989 * t15366;
    let t15371 = t3990 * t14116 * t3835;
    (t15345, t15346, t15348, t15357, t15358, t15366, t15367, t15371)
}
