//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2196/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196(t1442: f64, t1869: f64, t19289: f64, t25958: f64, t33085: f64, t4073: f64, t6287: f64, t6515: f64, t672: f64, t96686: f64, t97862: f64, t97865: f64, t97869: f64, t97871: f64, t97874: f64, t97878: f64, t97880: f64, t97887: f64, t97889: f64, t97892: f64, t97893: f64, t97897: f64, t97899: f64, t97905: f64) -> f64 {
    let t97906 = -2.0_f64 * t1442 * t25958 - t1869 * t19289 - 4.0_f64 * t33085 * t4073 - t6287 * t6515 - 2.0_f64 * t672 * t96686 - t97862 - t97865 - t97869 - t97871 + t97874 - t97878 + t97880 + t97887 - t97889 + t97892 - t97893 + t97897 + t97899 - t97905;
    t97906
}
