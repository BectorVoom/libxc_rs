//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1101/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1101(t2352: f64, t2416: f64, t353: f64, t859: f64, t938: f64, t19672: f64, t19677: f64, t19679: f64, t19683: f64, t19691: f64, t19696: f64, t19701: f64, t19704: f64, t2359: f64, t2362: f64, t2387: f64, t2388: f64, t4409: f64, t6151: f64, t6784: f64, t6789: f64, t6793: f64, t6797: f64, t827: f64) -> f64 {
    let t19710 = t859 * t353 * t2416 * t2352 * t938;
    let t19713 = 3.0_f64 / 8.0_f64 * t2388 * t6151 - t827 * t19672 / 4.0_f64 - t2388 * t6789 / 8.0_f64 + 7.0_f64 / 12.0_f64 * t19677 - 35.0_f64 / 36.0_f64 * t19679 - t2359 * t19683 / 24.0_f64 - t2388 * t6784 / 8.0_f64 - t2387 * t4409 * t2362 / 16.0_f64 + 7.0_f64 / 24.0_f64 * t19691 - t827 * t19696 - 3.0_f64 / 4.0_f64 * t6793 * t19701 + t19704 * t6797 / 4.0_f64 + t6793 * t19710 / 4.0_f64;
    t19713
}
