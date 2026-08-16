//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 809/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk809<F: Float>(t2206: F, t2216: F, t4379: F, t858: F, t886: F, t884: F, t2170: F, t6177: F, t6287: F, t3138: F, t346: F, t4408: F) -> (F, F, F, F, F, F) {
    let t6691 = t2206 * t2216;
    let t6692 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6691;
    let t6694 = t886 * t858 * t4379;
    let t6696 = t884 * t6694 / F::cast_from(48.0_f64);
    let t6698 = t2170 * t6177 * t6287;
    let t6700 = t3138 * t6698 / F::cast_from(8.0_f64);
    let t6701 = t4408 * t346;
    (t6692, t6694, t6696, t6698, t6700, t6701)
}
