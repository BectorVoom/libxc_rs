//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1381/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1381<F: Float>(t3909: F, t3955: F, t13796: F, t13859: F, t3896: F, t875: F, t1118: F, t3166: F, t3854: F, t898: F, t13798: F, t3989: F) -> (F, F, F, F) {
    let t57707 = t3955 * t3909;
    let t57711 = t13859 * t13796 * t3896 * t875;
    let t57719 = t13859 * t13796 * t1118 * t3166;
    let t57728 = t898 * t3854;
    let t57731 = t3989 * t13796 * t57728 * t13798;
    (t57707, t57711, t57719, t57731)
}
