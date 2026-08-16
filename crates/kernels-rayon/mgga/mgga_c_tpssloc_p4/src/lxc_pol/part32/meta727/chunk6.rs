//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2359/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2359(t1458: f64, t20127: f64, t2314: f64, t27858: f64, t27863: f64, t29501: f64, t29848: f64, t4034: f64, t4077: f64, t652: f64, t671: f64, t7266: f64, t97820: f64, t97829: f64, t97831: f64, t97833: f64, t97835: f64, t97836: f64, t97839: f64, t97842: f64, t97844: f64, t97846: f64, t97848: f64, t97850: f64, t97854: f64) -> f64 {
    let t105062 = -4.0_f64 * t1458 * t27858 * t652 - 2.0_f64 * t29848 * t652 * t671 - 2.0_f64 * t20127 * t7266 - 4.0_f64 * t2314 * t29501 - 4.0_f64 * t27863 * t4077 - 4.0_f64 * t29501 * t4034 + t97820 - t97829 - t97831 - t97833 + t97835 - t97836 + t97839 + t97842 - t97844 - t97846 - t97848 - t97850 - t97854;
    t105062
}
