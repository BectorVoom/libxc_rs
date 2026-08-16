//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 347/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk347(t322: f64, t1292: f64, t1295: f64, t1300: f64, t327: f64, t833: f64, t834: f64, t330: f64, t837: f64, t1291: f64) -> (f64, f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t1305 = -0.64e0_f64 * t1292 * t327 - 0.128e1_f64 * t1295 * t833 - 0.128e1_f64 * t1300 * t1295 - 0.64e0_f64 * t834 * t1292;
    let t1306 = t1305 * t330;
    let t1307 = t837 * t837;
    let t1308 = t1307 * t330;
    let t1310 = piecewise3(t332, 0.0_f64, t1291);
    (t1305, t1306, t1307, t1308, t1310)
}
