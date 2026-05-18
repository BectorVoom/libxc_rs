//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1355/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1355<F: Float>(t11737: F, t14637: F, t3974: F, t3990: F, t13796: F, t3887: F, t3989: F, t875: F, t376: F, t3854: F, t13859: F, t2171: F) -> (F, F, F, F) {
    let t57311 = t14637 * t3990 * t3974 * t11737;
    let t57319 = t3989 * t13796 * t3887 * t875;
    let t57321 = t376 * t3854;
    let t57324 = t13859 * t13796 * t57321 * t2171;
    (t57311, t57319, t57321, t57324)
}
