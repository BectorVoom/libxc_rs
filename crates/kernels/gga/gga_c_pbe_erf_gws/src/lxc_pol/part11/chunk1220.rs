//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1220/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1220<F: Float>(t44629: F, t44672: F, t2157: F, t3717: F, t11478: F, t2170: F, t3138: F, t13347: F, t2168: F, t13334: F, t3131: F, t3139: F) -> (F, F, F, F, F, F) {
    let t49371 = F::new(7.0) / F::new(24.0) * t44629;
    let t49372 = F::new(7.0) / F::new(12.0) * t44672;
    let t49374 = t2157 * t3717;
    let t49378 = t3138 * t2170 * t11478 * t49374 / F::new(4.0);
    let t49382 = t2168 * t2170 * t11478 * t13347 / F::new(8.0);
    let t49387 = t3138 * t3139 * t3131 * t2157 * t13334 / F::new(12.0);
    (t49371, t49372, t49374, t49378, t49382, t49387)
}
