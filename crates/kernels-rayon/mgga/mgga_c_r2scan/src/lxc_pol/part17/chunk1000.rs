//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1000/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1000(t1020: f64, t3524: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t12285: f64, t12286: f64, t12288: f64, t12292: f64, t12294: f64, t12296: f64, t12298: f64, t12300: f64, t12302: f64, t2412: f64, t343: f64) -> f64 {
    let t12305 = t3524 * t1020;
    let t12307 = 0.734774460522e2_f64 * t1129 * t2412 - 0.11494261417236e3_f64 * t1131 * t2412 + 0.6202613620464e2_f64 * t1133 * t2412 - 0.1088826475632e2_f64 * t1135 * t2412 - 0.957855118103e1_f64 * t12286 + 0.3101306810232e1_f64 * t12288 - 0.362942158544e0_f64 * t343 * t12285 - 0.8704e0_f64 * t12292 - 0.8704e0_f64 * t12294 - 0.8704e0_f64 * t12296 - 0.8704e0_f64 * t12298 - 0.4607056813647e1_f64 * t12300 + 0.122462410087e2_f64 * t12302 - 0.64e0_f64 * t12285 - 0.9214113627294e1_f64 * t12305;
    t12307
}
