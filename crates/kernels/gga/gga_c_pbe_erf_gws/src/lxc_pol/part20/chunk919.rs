//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 919/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk919<F: Float>(t10247: F, t10252: F, t10265: F, t10270: F, t10239: F, t10245: F, t145: F, t5726: F, t5730: F, t5732: F, t5733: F, t8347: F, t8351: F, t8371: F, t8373: F) -> (F, F) {
    let t10272 = t10247 + t10252 + t10265 + t10270;
    let t10275 = -F::new(0.31835665774679373271e-1) * t10239 - t8371 - F::new(0.63671331549358746542e-1) * t8373 - F::new(0.31835665774679373271e-1) * t5726 - t5730 - t5732 + F::new(0.3199504064530762818e0) * t5733 + F::new(0.6399008129061525636e0) * t8347 - t8351 - F::new(0.1066501354843587606e0) * t10245 + F::new(0.533250677421793803e-1) * t145 * t10272;
    (t10272, t10275)
}
