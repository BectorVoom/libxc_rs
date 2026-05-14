//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 511/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk511<F: Float>(t2797: F, t2014: F, t3481: F, t3490: F, t3495: F, t3496: F, t3498: F, t3502: F, t3506: F, t3507: F, t3508: F, t3509: F, t1044: F, t2607: F, t1621: F, t1620: F) -> (F, F, F, F, F) {
    let t3510 = 16.0 / 45.0 * t2797;
    let t3511 = t3481 + t3490 + t3495 + t3496 + t3498 - t3502 - t3506 - t3507 + t3508 - t3509 + t2014 + t3510;
    let t3512 = t2607 * t1044;
    let t3513 = t1621 * t3512;
    let t3515 = 8.0 / 15.0 * t1620 * t3513;
    (t3510, t3511, t3512, t3513, t3515)
}
