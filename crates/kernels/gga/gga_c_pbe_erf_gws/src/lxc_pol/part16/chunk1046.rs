//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1046/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1046<F: Float>(t3291: F, t6416: F, t2312: F, t8943: F, t8948: F, t8951: F, t8952: F, t8954: F, t8958: F, t9457: F, t9460: F, t9464: F, t9467: F, t9470: F) -> F {
    let t9474 = F::new(7.0) / F::new(1152.0) * t6416 * t3291;
    let t9475 = -F::new(119.0) / F::new(6912.0) * t9457 - t8943 + t8948 - t2312 * t9460 / F::new(192.0) - t8951 + t8952 - t9464 - t2312 * t9467 / F::new(384.0) - t8954 - t2312 * t9470 / F::new(384.0) - t8958 + t9474;
    t9475
}
