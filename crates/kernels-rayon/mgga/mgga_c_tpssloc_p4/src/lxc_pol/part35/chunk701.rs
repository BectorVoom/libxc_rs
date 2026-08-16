//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 701/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk701(t1375: f64, t1843: f64, t5215: f64, t5321: f64, t568: f64, t6362: f64, t6364: f64, t6435: f64, t6440: f64, t6461: f64) -> f64 {
    let t6463 = 2.0_f64 * t1375 * t6440 - t1375 * t6461 - 2.0_f64 * t1843 * t5215 - 2.0_f64 * t1843 * t5321 + t568 * t6362 + 2.0_f64 * t568 * t6364 + t568 * t6435;
    t6463
}
