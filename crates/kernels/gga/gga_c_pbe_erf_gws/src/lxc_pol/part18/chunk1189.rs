//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1189/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1189<F: Float>(t13917: F, t3223: F, t57432: F, t12014: F, t13919: F, t1115: F, t13911: F, t13930: F, t14397: F, t15273: F, t2498: F, t335: F, t338: F, t35057: F, t4002: F, t53617: F, t53939: F, t54488: F, t57402: F, t57404: F, t57410: F, t57415: F, t57422: F, t57428: F, t8629: F, t8793: F, t892: F, t9858: F) -> (F,) {
    let t57434 = t13917 * t57432 * t3223;
    let t57441 = t13917 * t13919 * t12014;
    let t57445 = t57402 / 24.0 + t57404 / 24.0 - t2498 * t14397 / 48.0 - t57410 / 192.0 - t1115 * t54488 / 48.0 - t57415 / 192.0 - t9858 * t4002 / 96.0 - t57422 / 1536.0 - t335 * t338 * t892 * t15273 / 96.0 + 7.0 / 144.0 * t57428 + t35057 * t13911 / 48.0 - t57434 / 1536.0 + t8629 * t53939 / 48.0 + t35057 * t13930 / 48.0 - t57441 / 1536.0 + t8793 * t53617 / 24.0;
    (t57445,)
}
