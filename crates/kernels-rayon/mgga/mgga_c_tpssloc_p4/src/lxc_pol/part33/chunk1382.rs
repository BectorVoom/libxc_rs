//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1382/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1382(t1799: f64, t22633: f64, t22635: f64, t97608: f64, t1985: f64, t20661: f64, t6889: f64, t6906: f64, t1375: f64, t20044: f64, t2015: f64, t2016: f64, t20608: f64, t28111: f64, t40591: f64, t5321: f64, t74908: f64, t7729: f64, t80744: f64, t90642: f64, t90659: f64, t90663: f64, t97509: f64) -> f64 {
    let t107031 = t22633 * t22635 * t97608 * t1799;
    let t107044 = t1985 * t6889 * t6906 * t20661;
    let t107048 = -3.0_f64 * t74908 * t2016 - t80744 + 0.24674011002723396548e-1_f64 * t97509 - 0.9869604401089358619e-1_f64 * t107031 + 24.0_f64 * t1375 * t40591 * t2015 * t20608 + 6.0_f64 * t5321 * t28111 + 0.24674011002723396547e-1_f64 * t90642 - 0.19190897446562641759e0_f64 * t90659 - 0.24674011002723396547e-1_f64 * t90663 - 0.82246703342411321825e-2_f64 * t107044 + 6.0_f64 * t20044 * t7729;
    t107048
}
