//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1236/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1236<F: Float>(t3065: F, t49529: F, t858: F, t8978: F, t13285: F, t2118: F, t2277: F, t3257: F, t44970: F, t44977: F, t45574: F, t49474: F, t49528: F, t49538: F, t49540: F, t49545: F, t49550: F, t6158: F, t6637: F, t9499: F) -> (F, F) {
    let t49555 = t8978 * t3065 * t858 * t49529 / F::cast_from(16.0_f64);
    let t49556 = t2277 * t3257 * t45574 * t13285 / F::cast_from(192.0_f64) + t49528 + t6637 * t9499 * t2118 * t49529 / F::cast_from(128.0_f64) + t49538 - t49540 - t6637 * t9499 * t6158 * t49474 / F::cast_from(96.0_f64) - t49545 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t44970 + t49550 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t44977 + t49555;
    (t49555, t49556)
}
