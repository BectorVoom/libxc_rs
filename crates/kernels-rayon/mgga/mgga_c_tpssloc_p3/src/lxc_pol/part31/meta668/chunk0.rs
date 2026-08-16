//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1965/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965(t225: f64, t29095: f64, t26729: f64, t866: f64, t86930: f64, t86931: f64, t92415: f64, t92425: f64, t98202: f64, t98205: f64, t98213: f64, t98222: f64, t98227: f64, t98279: f64) -> f64 {
    let t101355 = t29095 * t225;
    let t101359 = -0.3289868133696452873e-1_f64 * t98202 + 0.19739208802178717238e0_f64 * t98205 - 12.0_f64 * t98279 * t26729 - t92415 + t86930 - t86931 - 0.3289868133696452873e-1_f64 * t98213 - t101355 * t866 + 0.6579736267392905746e-1_f64 * t98222 - 0.9869604401089358619e-1_f64 * t98227 + t92425;
    t101359
}
