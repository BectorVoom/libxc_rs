//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 672/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk672<F: Float>(t3770: F, t3771: F, t1076: F, t2107: F, t3030: F, t323: F, t3373: F, t818: F) -> (F, F, F) {
    let t3772 = t3770 + t3771;
    let t3776 = t1076 * t1076;
    let t3780 = -F::cast_from(2.0_f64) * t1076 * t3030 + F::cast_from(2.0_f64) * t2107 * t3776 + t323 * t3772 - t3373 * t818;
    (t3772, t3776, t3780)
}
