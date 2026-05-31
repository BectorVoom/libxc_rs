//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 656/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk656<F: Float>(t418: F, t92: F, t422: F, t93: F, t108: F, t1407: F, t1416: F, t4352: F, t4360: F, t4367: F, t4373: F, t476: F, t478: F, t726: F, t728: F) -> (F, F, F) {
    let t5189 = t92 * t418;
    let t5196 = t93 * t422;
    let t5202 = (F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t476 * t4352 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t5189 * t1407 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t4360 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t478 * t4367 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t5196 * t1416 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t4373) * t108;
    (t5189, t5196, t5202)
}
