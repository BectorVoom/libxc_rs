//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1220/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1220<F: Float>(t15651: F, t191: F, t22: F, t364: F, t369: F, t371: F, t21419: F, t2168: F, t3139: F, t875: F, t6222: F, t6484: F) -> (F, F, F) {
    let t21647 = F::new(13685.0) / F::new(31104.0) * t364 / t22 / t15651 * t191 * t369 * t371;
    let t21651 = t2168 * t3139 * t21419 * t875 / F::new(24.0);
    let t21652 = t6484 * t6222;
    (t21647, t21651, t21652)
}
