//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1194/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1194<F: Float>(t1161: F, t4182: F, t2409: F, t3067: F, t3855: F, t3990: F, t3991: F, t3989: F, t14116: F, t3835: F, t1173: F, t3909: F) -> (F, F, F, F, F, F, F) {
    let t15360 = t4182 * t1161;
    let t15362 = t2409 * t3067 * t15360;
    let t15366 = t3990 * t3991 * t3855;
    let t15367 = t3989 * t15366;
    let t15371 = t3990 * t14116 * t3835;
    let t15372 = t3989 * t15371;
    let t15374 = t1173 * t3909;
    (t15360, t15362, t15366, t15367, t15371, t15372, t15374)
}
