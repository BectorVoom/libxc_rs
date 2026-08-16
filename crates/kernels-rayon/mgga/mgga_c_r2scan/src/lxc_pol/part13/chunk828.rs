//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 828/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk828(t2591: f64, t7433: f64, t360: f64, t2572: f64, t6127: f64, t2573: f64, t2133: f64, t2598: f64, t5123: f64, t5126: f64, t5130: f64, t5144: f64, t5150: f64, t5164: f64, t5170: f64, t5175: f64, t5179: f64, t5183: f64, t5186: f64, t5189: f64, t7419: f64, t7430: f64) -> (f64, f64, f64, f64) {
    let t7434 = t7433 * t2591;
    let t7435 = t360 * t7434;
    let t7438 = t2572 * t6127;
    let t7439 = t360 * t7438;
    let t7442 = t7433 * t2573;
    let t7443 = t360 * t7442;
    let t7446 = 0.11708928647259339622e0_f64 * t5123 - 0.97574405393827830186e-2_f64 * t5126 - 0.58218257753910989057e-2_f64 * t5130 - 0.23115257973478049502e0_f64 * t5144 + 0.16262400898971305031e-3_f64 * t5150 - 0.28914548798370980346e-3_f64 * t7419 - 0.17465477326173296717e-1_f64 * t5164 + 0.27439371595564631661e-2_f64 * t5170 - 0.54878743191129263322e-2_f64 * t5175 + 0.1358426014257923078e0_f64 * t5179 + 0.4075278042773769234e0_f64 * t5183 - 0.11643651550782197811e-1_f64 * t5186 - 0.34930954652346593434e-1_f64 * t5189 + 0.43341108700271342816e-1_f64 * t2133 * t7430 + 0.17336443480108537126e0_f64 * t2598 * t7435 + 0.86682217400542685632e-1_f64 * t2598 * t7439 + 0.86682217400542685632e-1_f64 * t2133 * t7443;
    (t7434, t7438, t7442, t7446)
}
