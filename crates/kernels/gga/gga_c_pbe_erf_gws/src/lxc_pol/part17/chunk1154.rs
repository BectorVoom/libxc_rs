//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1154/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1154<F: Float>(t14046: F, t3184: F, t51408: F, t3148: F, t3123: F, t51309: F, t14015: F, t9467: F, t14023: F, t14548: F, t863: F, t51412: F, t14547: F, t28029: F, t6523: F, t14031: F, t9556: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54319 = t14046 * t3184;
    let t54320 = 7.0 / 72.0 * t54319;
    let t54321 = 35.0 / 216.0 * t51408;
    let t54322 = t14046 * t3148;
    let t54323 = 7.0 / 72.0 * t54322;
    let t54324 = t3123 * t51309;
    let t54326 = t14015 * t9467;
    let t54329 = t863 * t14023 * t14548;
    let t54330 = 7.0 / 24.0 * t54329;
    let t54331 = 35.0 / 108.0 * t51412;
    let t54333 = t14547 * t6523 * t28029;
    let t54335 = t14031 * t9556;
    (t54320, t54321, t54323, t54324, t54326, t54330, t54331, t54333, t54335)
}
