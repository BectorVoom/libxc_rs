//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1191/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1191(t37982: f64, t9373: f64, t11654: f64, t7601: f64, t10743: f64, t3198: f64, t261: f64, t3299: f64, t9451: f64, t31060: f64, t3333: f64, t12533: f64, t22796: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43230 = t37982 * t9373;
    let t43232 = t7601 * t11654;
    let t43234 = t10743 * t3198;
    let t43238 = t3299 * t261 * t9451;
    let t43240 = t31060 * t3333;
    let t43242 = t22796 * t12533;
    (t43230, t43232, t43234, t43238, t43240, t43242)
}
