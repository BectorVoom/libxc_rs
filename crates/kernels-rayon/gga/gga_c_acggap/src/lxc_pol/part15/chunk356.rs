//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 356/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk356(t119: f64, t1215: f64, t1306: f64, t1309: f64, t1605: f64, t1608: f64, t1611: f64, t1615: f64, t1620: f64, t1659: f64, t446: f64, t464: f64, t557: f64, t850: f64, t854: f64, t855: f64, t858: f64, t867: f64, t869: f64, t873: f64, t882: f64) -> f64 {
    let t1662 = t850 - t854 + 0.65854491829355115987e0_f64 * t855 - 0.65854491829355115987e0_f64 * t858 + t867 - 0.65854491829355115987e0_f64 * t869 + 0.65854491829355115987e0_f64 * t873 - t882 + 0.65854491829355115987e0_f64 * t1306 - 0.65854491829355115987e0_f64 * t1309 + 0.65854491829355115987e0_f64 * t119 * t1605 - 0.65854491829355115987e0_f64 * t1608 * t464 - 0.65854491829355115987e0_f64 * t1611 + 0.65854491829355115987e0_f64 * t1615 - 0.65854491829355115987e0_f64 * t1215 * t557 + 0.13170898365871023197e1_f64 * t446 * t1620 - 0.65854491829355115987e0_f64 * t446 * t1659;
    t1662
}
