//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1243/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1243(t45381: f64, t2168: f64, t2170: f64, t3814: f64, t44741: f64, t45400: f64, t1134: f64, t11700: f64, t13156: f64, t13544: f64, t21519: f64, t2255: f64, t2277: f64, t2343: f64, t274: f64, t3258: f64, t3757: f64, t45200: f64, t45345: f64, t45353: f64, t45408: f64, t45574: f64, t49681: f64, t6579: f64) -> (f64, f64, f64, f64) {
    let t49683 = 7.0_f64 / 12.0_f64 * t45381;
    let t49687 = t2168 * t2170 * t44741 * t3814 / 12.0_f64;
    let t49696 = 7.0_f64 / 24.0_f64 * t45400;
    let t49703 = 5.0_f64 / 32.0_f64 * t6579 * t11700 * t13544 + 7.0_f64 / 96.0_f64 * t45345 - t49681 + 7.0_f64 / 96.0_f64 * t45353 - t49683 + t49687 + 5.0_f64 / 16.0_f64 * t2343 * t21519 * t45200 * t1134 - t2277 * t2255 * t45574 * t3757 / 512.0_f64 + t49696 - t2277 * t2255 * t3258 * t274 * t13156 / 1536.0_f64 - 7.0_f64 / 96.0_f64 * t45408;
    (t49683, t49687, t49696, t49703)
}
