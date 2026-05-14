//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 685/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk685<F: Float>(t1185: F, t4033: F, t1184: F, t888: F, t367: F, t864: F, t899: F) -> (F, F, F) {
    let t4034 = t4033 * t1185;
    let t4035 = 7.0 / 288.0 * t4034;
    let t4036 = t1184 * t888;
    let t4039 = t899 * t864 * t367;
    (t4035, t4036, t4039)
}
