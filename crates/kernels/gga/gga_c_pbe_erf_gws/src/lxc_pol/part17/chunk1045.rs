//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1045/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1045<F: Float>(t3291: F, t6416: F, t2312: F, t8943: F, t8948: F, t8951: F, t8952: F, t8954: F, t8958: F, t9457: F, t9460: F, t9464: F, t9467: F, t9470: F) -> F {
    let t9474 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t6416 * t3291;
    let t9475 = -F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t9457 - t8943 + t8948 - t2312 * t9460 / F::cast_from(192.0_f64) - t8951 + t8952 - t9464 - t2312 * t9467 / F::cast_from(384.0_f64) - t8954 - t2312 * t9470 / F::cast_from(384.0_f64) - t8958 + t9474;
    t9475
}
