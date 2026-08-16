//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2290/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290(t28525: f64, t344: f64, t6740: f64, t5904: f64, t6764: f64, t1046: f64, t17681: f64, t17890: f64, t23419: f64, t23483: f64, t23544: f64, t28526: f64, t28578: f64, t28582: f64, t28587: f64, t5857: f64, t5861: f64, t6735: f64, t6747: f64, t6765: f64, t83117: f64, t83121: f64, t88548: f64) -> f64 {
    let t99720 = t6740 * t28525 * t344;
    let t99731 = t5904 * t6764;
    let t99736 = t23419 * t17681 / 2304.0_f64 - 0.80745512188280781712e-3_f64 * t23483 * t28587 - 0.16149102437656156342e-2_f64 * t83121 * t28578 + 0.80745512188280781712e-3_f64 * t83121 * t28582 + 0.10093189023535097714e-3_f64 * t99720 * t6747 + 0.20186378047070195428e-3_f64 * t83117 * t28578 + t23544 * t5857 / 2304.0_f64 + t6765 * t17890 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t23544 * t5861 + t99731 * t1046 / 2304.0_f64 - t88548 - 0.10093189023535097714e-3_f64 * t28526 * t6735;
    t99736
}
