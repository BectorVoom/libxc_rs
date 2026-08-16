//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1243/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1243<F: Float>(t45381: F, t2168: F, t2170: F, t3814: F, t44741: F, t45400: F, t1134: F, t11700: F, t13156: F, t13544: F, t21519: F, t2255: F, t2277: F, t2343: F, t274: F, t3258: F, t3757: F, t45200: F, t45345: F, t45353: F, t45408: F, t45574: F, t49681: F, t6579: F) -> (F, F, F, F) {
    let t49683 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t45381;
    let t49687 = t2168 * t2170 * t44741 * t3814 / F::cast_from(12.0_f64);
    let t49696 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t45400;
    let t49703 = F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t6579 * t11700 * t13544 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t45345 - t49681 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t45353 - t49683 + t49687 + F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t2343 * t21519 * t45200 * t1134 - t2277 * t2255 * t45574 * t3757 / F::cast_from(512.0_f64) + t49696 - t2277 * t2255 * t3258 * t274 * t13156 / F::cast_from(1536.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t45408;
    (t49683, t49687, t49696, t49703)
}
