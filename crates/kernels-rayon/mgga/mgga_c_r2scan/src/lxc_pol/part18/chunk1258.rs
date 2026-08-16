//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1258/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1258(t797: f64, t8629: f64, t3262: f64, t3263: f64, t11479: f64, t11550: f64, t481: f64, t9573: f64, t38282: f64, t38298: f64, t38303: f64, t38312: f64, t38323: f64, t40642: f64, t40659: f64, t40672: f64, t43939: f64, t43943: f64, t43946: f64, t43949: f64) -> (f64, f64, f64, f64) {
    let t43950 = t797 * t8629;
    let t43953 = 3.0_f64 / 4.0_f64 * t3262 * t3263 * t43950;
    let t43958 = 3.0_f64 / 2.0_f64 * t3262 * t11479 * t11550;
    let t43959 = t9573 * t481;
    let t43962 = 3.0_f64 / 2.0_f64 * t3262 * t3263 * t43959;
    let t43963 = 0.60975299583150056628e-3_f64 * t40642 - t38282 + 0.68400385060046895006e-6_f64 * t40659 - 0.15243824895787514157e-3_f64 * t43939 + 0.21684485328539747656e-4_f64 * t43943 - t43946 - t43949 - t43953 - 0.70441376091769752087e-2_f64 * t40672 - t38298 - 0.72042316457491791906e-3_f64 * t38303 + t38312 - t43958 - t43962 + t38323;
    (t43953, t43958, t43962, t43963)
}
