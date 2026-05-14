//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 623/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk623<F: Float>(t108: F, t3342: F, t3346: F, t3351: F, t3354: F, t726: F, t728: F, t92: F, t93: F, t1902: F, t1905: F, t1920: F, t1926: F, t267: F, t3498: F, t3502: F, t3506: F, t3507: F, t3508: F, t3509: F, t3510: F) -> (F, F) {
    let t3603 = (20.0 / 9.0 * t92 * t3342 + 4.0 / 3.0 * t726 * t3346 + 20.0 / 9.0 * t93 * t3351 + 4.0 / 3.0 * t728 * t3354) * t108;
    let t3606 = t3498 - t3502 - t3506 - t3507 + t3508 - t3509 - t3603 * t267 / 15.0 + t3510 + t1902 - t1905 + t1920 + t1926;
    (t3603, t3606)
}
