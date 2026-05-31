//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 343/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk343<F: Float>(t1044: F, t650: F, t186: F, t211: F, t225: F, t991: F) -> (F, F, F, F) {
    let t1045 = t650 * t1044;
    let t1046 = t186 * t1045;
    let t1048 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t1046;
    let t1049 = t991 * t225;
    (t1045, t1046, t1048, t1049)
}
