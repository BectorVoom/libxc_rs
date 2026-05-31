//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 775/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk775<F: Float>(t320: F, t368: F, t191: F, t1: F, t2182: F, t810: F) -> (F, F, F, F) {
    let t6382 = F::cast_from(1.0_f64) / t368 / t320;
    let t6383 = t191 * t6382;
    let t6384 = t6383 * t1;
    let t6385 = t2182 * t810;
    (t6382, t6383, t6384, t6385)
}
