//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 705/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk705<F: Float>(t4049: F, t935: F, t4024: F, t4030: F, t4035: F, t4036: F, t4040: F, t4044: F, t4047: F) -> F {
    let t4050 = t4049 * t935;
    let t4052 = t4024 / F::new(96.0) - t4030 / F::new(96.0) - t4035 - t4036 / F::new(48.0) + t4040 / F::new(768.0) - t4044 / F::new(768.0) - t4047 - t4050 / F::new(384.0);
    t4052
}
