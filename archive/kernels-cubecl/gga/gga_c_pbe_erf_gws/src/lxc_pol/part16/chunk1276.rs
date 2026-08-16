//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1276/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1276<F: Float>(t14069: F, t9111: F, t14064: F, t3108: F, t14031: F, t9348: F, t14011: F, t9666: F, t14538: F, t51329: F, t4028: F, t9131: F) -> (F, F, F, F, F, F) {
    let t54158 = t9111 * t14069;
    let t54160 = t3108 * t14064;
    let t54162 = t14031 * t9348;
    let t54164 = t14011 * t9666;
    let t54166 = t14538 * t51329;
    let t54168 = t4028 * t9131;
    (t54158, t54160, t54162, t54164, t54166, t54168)
}
