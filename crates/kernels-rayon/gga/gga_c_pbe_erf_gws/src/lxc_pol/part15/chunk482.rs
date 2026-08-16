//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 482/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk482(t125: f64, t143: f64, t1452: f64, t1499: f64, t1501: f64, t1503: f64, t1504: f64, t1593: f64, t169: f64, t1944: f64, t2024: f64, t2026: f64, t2033: f64, t2035: f64, t2037: f64, t2042: f64, t279: f64, t299: f64, t301: f64, t475: f64, t523: f64, t526: f64) -> f64 {
    let t2048 = -t1499 + t523 * t1501 + 6.0_f64 * t1503 * t143 * t1504 + t1593 * t526 + t1944 * t279 + t2024 * t125 + 3.0_f64 * t475 * t2026 - t523 * t2033 + 6.0_f64 * t2035 * t2037 - 0.10809180959278284142e0_f64 * t2042 + 0.20267214298646782767e-1_f64 * t169 * t299 * t1452 * t301;
    t2048
}
