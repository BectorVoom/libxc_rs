//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1045/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1045<F: Float>(t2147: F, t337: F, t44313: F, t13481: F, t2319: F, t13069: F, t19: F, t796: F, t801: F, t13156: F, t817: F, t13536: F, t2142: F) -> (F, F, F, F, F) {
    let t44315 = t2147 * t337 * t44313;
    let t44372 = t2319 * t13481;
    let t44395 = t13069 * t796 * t19 * t801;
    let t44405 = t13156 * t817;
    let t44465 = t13536 * t2142;
    (t44315, t44372, t44395, t44405, t44465)
}
