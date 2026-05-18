//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1071/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1071<F: Float>(t19301: F, t496: F, t19259: F, t19264: F, t19266: F, t19270: F, t19274: F, t19279: F, t19282: F, t19286: F, t19290: F, t19294: F, t19296: F, t19299: F) -> F {
    let t19302 = t496 * t19301;
    let t19304 = -F::new(0.587616e1) * t19259 - t19264 + F::new(8.0) * t19266 + F::new(30.0) * t496 * t19270 + F::new(9.0) / F::new(2.0) * t496 * t19274 + F::new(0.2350464e2) * t19279 + t19282 + t19286 + t19290 + t19294 + F::new(56.0) / F::new(27.0) * t19296 - F::new(4.0) / F::new(3.0) * t19299 + F::new(2.0) / F::new(3.0) * t19302;
    t19304
}
