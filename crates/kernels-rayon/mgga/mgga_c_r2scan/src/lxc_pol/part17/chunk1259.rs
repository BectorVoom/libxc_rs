//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1259/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1259(t1020: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t12296: f64, t2410: f64, t2412: f64, t2958: f64, t3524: f64, t3526: f64, t3530: f64, t3534: f64, t3538: f64, t3747: f64, t3749: f64, t3753: f64, t3757: f64, t3761: f64, t9711: f64) -> f64 {
    let t44746 = 0.1469548921044e3_f64 * t3749 * t2412 + 0.734774460522e2_f64 * t1129 * t9711 - 0.22988522834472e3_f64 * t3753 * t2412 - 0.11494261417236e3_f64 * t1131 * t9711 + 0.12405227240928e3_f64 * t3757 * t2412 + 0.6202613620464e2_f64 * t1133 * t9711 - 0.2177652951264e2_f64 * t3761 * t2412 - 0.1088826475632e2_f64 * t1135 * t9711 + 0.734774460522e2_f64 * t3524 * t2958 + 0.734774460522e2_f64 * t3526 * t2958 - 0.11494261417236e3_f64 * t3530 * t2958 + 0.6202613620464e2_f64 * t3534 * t2958 - 0.1088826475632e2_f64 * t3538 * t2958 - 0.18428227254588e2_f64 * t3747 * t2410 - 0.18428227254588e2_f64 * t12296 * t1020;
    t44746
}
