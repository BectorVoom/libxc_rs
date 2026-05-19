//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 771/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk771<F: Float>(t1: F, t1952: F, t119: F, t713: F, t1805: F, t582: F, t185: F, t1472: F, t168: F, t738: F, t1931: F, t703: F) -> (F, F, F, F) {
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    let t5562 = F::cast_from(0.15154381759259259259e-2_f64) * t5559 * t5560;
    let t5563 = t582 * t1805;
    let t5564 = t185 * t5563;
    let t5574 = t168 * t1472 * t738;
    let t5577 = t168 * t703 * t1931;
    (t5562, t5564, t5574, t5577)
}
