//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1120/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1120<F: Float>(t14185: F, t2410: F, t9283: F, t2376: F, t4110: F, t829: F, t830: F) -> (F, F, F) {
    let t14321 = t14185 * t2410;
    let t14322 = t9283 * t14321;
    let t14325 = t2376 * t4110;
    let t14327 = t829 * t830 * t14325;
    (t14321, t14322, t14327)
}
