//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1060/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1060<F: Float>(t2272: F, t3252: F, t2253: F, t2312: F, t6275: F, t6628: F, t6637: F, t6656: F, t9142: F, t9143: F, t9145: F, t9174: F, t9601: F, t9604: F, t9609: F, t9612: F, t9616: F) -> (F, F) {
    let t9619 = t3252 * t2272;
    let t9623 = -t9601 - t9142 - t9143 - t9145 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6628 - t2253 * t9604 / F::cast_from(768.0_f64) + t6275 * t9609 / F::cast_from(96.0_f64) + t6637 * t9612 / F::cast_from(768.0_f64) - t6637 * t9616 / F::cast_from(384.0_f64) + t9174 - t2312 * t9619 / F::cast_from(384.0_f64) - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t6656;
    (t9619, t9623)
}
