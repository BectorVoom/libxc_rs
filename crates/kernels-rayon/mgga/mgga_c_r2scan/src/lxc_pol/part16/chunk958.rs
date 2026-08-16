//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 958/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk958(t11529: f64, t2847: f64, t797: f64, t3275: f64, t3276: f64, t3696: f64, t860: f64, t1044: f64, t3424: f64, t3685: f64, t885: f64, t4176: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11530 = 5.0_f64 / 16.0_f64 * t11529;
    let t11531 = t797 * t2847;
    let t11533 = t3275 * t3276 * t11531;
    let t11534 = 5.0_f64 / 16.0_f64 * t11533;
    let t11535 = t860 * t3696;
    let t11537 = t3424 * t1044;
    let t11538 = t3685 * t885;
    let t11539 = t4176 * t986;
    (t11530, t11531, t11533, t11534, t11535, t11537, t11538, t11539)
}
