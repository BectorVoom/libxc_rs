//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1294/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1294<F: Float>(t28652: F, t3808: F, t3972: F, t3975: F, t361: F, t56296: F, t13917: F, t3223: F, t13796: F, t14423: F, t3166: F, t3989: F) -> (F, F, F) {
    let t56678 = t3972 * t3975 * t3808 * t28652;
    let t56684 = t361 * t56296;
    let t56686 = t13917 * t56684 * t3223;
    let t56697 = t3989 * t13796 * t14423 * t3166;
    (t56678, t56686, t56697)
}
