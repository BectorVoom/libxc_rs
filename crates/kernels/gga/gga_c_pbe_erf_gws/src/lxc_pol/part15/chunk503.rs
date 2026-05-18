//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 503/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk503<F: Float>(t2142: F, t854: F, t346: F, t824: F, t822: F) -> (F, F, F, F) {
    let t2143 = t854 * t2142;
    let t2144 = F::new(7.0) / F::new(144.0) * t2143;
    let t2145 = t824 * t346;
    let t2146 = t822 * t2145;
    (t2143, t2144, t2145, t2146)
}
