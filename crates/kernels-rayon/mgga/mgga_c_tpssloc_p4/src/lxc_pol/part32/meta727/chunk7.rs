//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2360/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2360(t104729: f64, t104976: f64, t27290: f64, t4028: f64, t510: f64, t5361: f64, t5493: f64, t652: f64, t7408: f64, t8107: f64, t97856: f64, t97858: f64, t97862: f64, t97865: f64, t97869: f64, t97871: f64, t97874: f64, t97878: f64, t97880: f64, t97887: f64, t97889: f64, t97892: f64, t97893: f64, t97897: f64) -> f64 {
    let t105073 = -2.0_f64 * t5493 * t652 * t7408 - 2.0_f64 * t104729 * t510 - t104976 * t510 - 4.0_f64 * t27290 * t4028 + 2.0_f64 * t5361 * t8107 - t97856 - t97858 - t97862 - t97865 - t97869 - t97871 + t97874 - t97878 + t97880 + t97887 - t97889 + t97892 - t97893 + t97897;
    t105073
}
