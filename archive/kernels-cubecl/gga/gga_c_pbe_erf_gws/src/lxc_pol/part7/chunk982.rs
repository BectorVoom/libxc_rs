//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 982/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk982<F: Float>(t16520: F, t16522: F, t16525: F, t16527: F, t16529: F, t16537: F, t16566: F, t16597: F, t16599: F, t16601: F, t16603: F, t16605: F) -> F {
    let t18157 = -t16520 - t16522 + t16525 + t16527 + t16529 + t16537 + t16566 + t16597 + t16599 - t16601 - t16603 + t16605;
    t18157
}
