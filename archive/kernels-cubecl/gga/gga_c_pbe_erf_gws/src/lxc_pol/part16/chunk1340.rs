//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1340/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1340<F: Float>(t54267: F, t54271: F, t54283: F, t54285: F, t54289: F, t51372: F, t54265: F, t54269: F, t54273: F, t54276: F, t54280: F, t54287: F) -> F {
    let t55562 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t54267;
    let t55564 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54271;
    let t55569 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54283;
    let t55570 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54285;
    let t55572 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54289;
    let t55573 = -t54265 / F::cast_from(48.0_f64) + t55562 + t54269 / F::cast_from(24.0_f64) - t55564 + t54273 / F::cast_from(96.0_f64) + t54276 / F::cast_from(4.0_f64) - t54280 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51372 + t55569 - t55570 - t54287 / F::cast_from(384.0_f64) - t55572;
    t55573
}
