//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2330/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2330(t104355: f64, t104364: f64, t104367: f64, t104369: f64, t104371: f64, t104375: f64, t18215: f64, t2121: f64, t2132: f64, t2133: f64, t24736: f64, t27703: f64, t4899: f64, t6138: f64, t6203: f64, t7321: f64, t8027: f64, t95540: f64, t95542: f64, t95545: f64) -> f64 {
    let t104380 = 0.10093189023535097714e-3_f64 * t104355 - 0.10093189023535097714e-3_f64 * t2132 * t2133 * t6138 * t7321 + t2121 * t4899 * t18215 / 108.0_f64 - 0.20186378047070195428e-3_f64 * t104364 - 0.10093189023535097714e-3_f64 * t104367 - t104369 / 3456.0_f64 - t104371 / 1728.0_f64 + 5.0_f64 / 6912.0_f64 * t24736 * t6203 - t104375 / 1728.0_f64 + 0.16149102437656156342e-2_f64 * t8027 * t27703 * t7321 - t95540 + t95542 + t95545;
    t104380
}
