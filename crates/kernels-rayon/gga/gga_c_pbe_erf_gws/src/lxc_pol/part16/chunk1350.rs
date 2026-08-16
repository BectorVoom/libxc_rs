//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1350/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1350(t53970: f64, t53975: f64, t53985: f64, t54427: f64, t54429: f64, t14918: f64, t2220: f64, t2388: f64, t2392: f64, t335: f64, t338: f64, t4228: f64, t51864: f64, t51877: f64, t52514: f64, t52525: f64, t53973: f64, t53981: f64, t53983: f64) -> f64 {
    let t55739 = 7.0_f64 / 72.0_f64 * t53970;
    let t55741 = 7.0_f64 / 288.0_f64 * t53975;
    let t55745 = 7.0_f64 / 36.0_f64 * t53985;
    let t55751 = 119.0_f64 / 1728.0_f64 * t54427;
    let t55752 = 7.0_f64 / 72.0_f64 * t54429;
    let t55758 = -t55739 + t53973 / 8.0_f64 + t55741 + t53981 / 12.0_f64 + t53983 / 4.0_f64 + 7.0_f64 / 288.0_f64 * t52514 + t55745 - t335 * t338 * t2220 * t4228 / 96.0_f64 - 7.0_f64 / 36.0_f64 * t51864 - t55751 + t55752 - t52525 + 35.0_f64 / 108.0_f64 * t51877 - t2388 * t14918 / 96.0_f64 - t2392 * t14918 / 96.0_f64;
    t55758
}
