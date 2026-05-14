//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1128/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1128<F: Float>(t12213: F, t13622: F, t21807: F, t2409: F, t3066: F, t338: F, t353: F, t36244: F, t36246: F, t376: F, t3917: F, t44104: F, t46703: F, t46710: F, t46712: F, t46714: F, t46717: F, t46723: F, t46731: F, t50002: F, t8793: F, t9890: F) -> (F,) {
    let t50642 = 7.0 / 36.0 * t46703 - t3917 * t9890 / 8.0 + 7.0 / 24.0 * t46710 + 7.0 / 24.0 * t46712 - 7.0 / 24.0 * t46714 - 7.0 / 4.0 * t46717 + 7.0 / 36.0 * t46723 - 7.0 / 12.0 * t46731 + t3066 * t2409 * t12213 * t13622 / 4.0 + 5.0 / 4.0 * t21807 * t338 * t353 * t376 * t50002 + 35.0 / 36.0 * t36244 - 35.0 / 18.0 * t36246 + t8793 * t44104 / 2.0;
    (t50642,)
}
