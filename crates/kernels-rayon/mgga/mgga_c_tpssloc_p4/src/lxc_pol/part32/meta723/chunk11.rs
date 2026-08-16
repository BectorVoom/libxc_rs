//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2317/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2317(t18375: f64, t7339: f64, t27599: f64, t4997: f64, t18360: f64, t18364: f64, t18397: f64, t18401: f64, t19002: f64, t19016: f64, t24741: f64, t27617: f64, t4950: f64, t4980: f64, t4984: f64, t5014: f64, t5030: f64, t86324: f64, t86327: f64, t95566: f64, t95623: f64, t95627: f64) -> f64 {
    let t104048 = t7339 * t18375;
    let t104050 = t27599 * t4997;
    let t104056 = t95566 * t4950 / 216.0_f64 - t24741 * t18360 / 1152.0_f64 + 5.0_f64 / 6912.0_f64 * t24741 * t18364 - t27599 * t5014 / 144.0_f64 - t27617 * t5030 / 1152.0_f64 + 5.0_f64 / 3456.0_f64 * t24741 * t19016 - t86324 * t19002 / 576.0_f64 + t86327 * t18397 / 1152.0_f64 - t24741 * t18401 / 576.0_f64 + t104048 / 2304.0_f64 - t104050 / 216.0_f64 - t95623 * t4980 / 72.0_f64 + t95627 * t4984 / 144.0_f64;
    t104056
}
