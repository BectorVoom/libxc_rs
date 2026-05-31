//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1387/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1387<F: Float>(t55562: F, t55564: F, t55569: F, t55570: F, t55572: F, t57108: F, t57110: F, t57112: F, t57114: F, t57117: F, t57119: F, t57121: F, t57123: F) -> F {
    let t58719 = -t57108 / F::cast_from(192.0_f64) - t57110 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57112 + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t57114 - t57117 / F::cast_from(4.0_f64) + t55562 + t57119 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57121 + t57123 / F::cast_from(24.0_f64) - t55564 + t55569 - t55570 - t55572;
    t58719
}
