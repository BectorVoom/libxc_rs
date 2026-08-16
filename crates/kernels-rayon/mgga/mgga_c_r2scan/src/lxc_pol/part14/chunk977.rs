//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 977/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk977(t11397: f64, t11413: f64, t11430: f64, t11447: f64, t797: f64, t1048: f64, t499: f64, t11017: f64, t10634: f64, t3472: f64, t3262: f64, t11011: f64, t3465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11449 = t11397 + t11413 + t11430 + t11447;
    let t11450 = t11449 * t797;
    let t11452 = t1048 * t499 * t11450;
    let t11453 = t11452 / 4.0_f64;
    let t11454 = 0.39032073591371545778e-3_f64 * t11017;
    let t11455 = t3472 * t10634;
    let t11456 = t3262 * t11455;
    let t11457 = 15.0_f64 / 8.0_f64 * t11456;
    let t11458 = t3465 * t11011;
    (t11449, t11450, t11453, t11454, t11455, t11457, t11458)
}
