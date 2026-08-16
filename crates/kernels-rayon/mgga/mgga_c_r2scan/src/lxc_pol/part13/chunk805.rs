//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 805/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk805(t1422: f64, t899: f64, t1416: f64, t1419: f64, t7055: f64, t7058: f64, t7091: f64, t7093: f64, t7095: f64, t7097: f64, t7098: f64, t7101: f64, t7104: f64, t881: f64) -> (f64, f64, f64, f64) {
    let t7107 = t1422 * t899;
    let t7108 = 32.0_f64 * t7107;
    let t7109 = t1416 * t899;
    let t7110 = 20.0_f64 * t7109;
    let t7111 = t1419 * t899;
    let t7112 = 12.0_f64 * t7111;
    let t7113 = -t7055 - t7058 - t7091 - t7093 - t7095 + t7097 - 0.2363e1_f64 * t881 * t7098 - 0.4726e1_f64 * t881 * t7101 - 0.2363e1_f64 * t881 * t7104 + t7108 - t7110 - t7112;
    (t7108, t7110, t7112, t7113)
}
