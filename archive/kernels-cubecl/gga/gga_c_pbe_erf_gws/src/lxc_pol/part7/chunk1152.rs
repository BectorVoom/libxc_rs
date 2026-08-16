//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1152/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1152<F: Float>(t20571: F, t20576: F, t20582: F, t20584: F, t20588: F, t20593: F, t20594: F, t20601: F, t20606: F, t20607: F, t20608: F, t20615: F, t2255: F, t2277: F, t6276: F, t6665: F) -> F {
    let t20616 = t2277 * t2255 * t20571 * t6665 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t20576 - t20582 + t20584 - t20588 - t20593 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t20594 + t20601 - t20606 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t20607 * t6276 * t20608 + t20615;
    t20616
}
