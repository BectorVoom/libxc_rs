//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 685/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk685<F: Float>(t1130: F, t2181: F, t3154: F, t339: F, t340: F, t3772: F, t3848: F, t3851: F, t870: F) -> F {
    let t3854 = -t339 * t340 * t3772 + F::new(6.0) * t1130 * t3154 - F::new(12.0) * t2181 * t3848 + F::new(3.0) * t3851 * t870;
    t3854
}
