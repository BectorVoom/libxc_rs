//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1002/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1002<F: Float>(t14257: F, t14298: F, t14332: F, t14358: F, t2053: F, t4116: F, t944: F, t1211: F, t6854: F, t2051: F, t2423: F, t4120: F, t1105: F, t13919: F, t3227: F, t13917: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14360 = t14257 + t14298 + t14332 + t14358;
    let t14364 = t4116 * t2053;
    let t14365 = t14364 * t944;
    let t14368 = t1211 * t6854;
    let t14369 = t14368 * t2051;
    let t14372 = t4120 * t2423;
    let t14383 = t1105 * t944;
    let t14415 = t13919 * t3227;
    let t14416 = t13917 * t14415;
    (t14360, t14364, t14365, t14368, t14369, t14372, t14383, t14415, t14416)
}
