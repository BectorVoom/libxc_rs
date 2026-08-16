//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 979/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk979(t11494: f64, t2333: f64, t910: f64, t795: f64, t3263: f64, t3262: f64, t1065: f64, t3270: f64, t10667: f64, t105: f64, t920: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11495 = t11494 / 4.0_f64;
    let t11496 = t2333 * t910;
    let t11497 = t11496 * t795;
    let t11498 = t3263 * t11497;
    let t11499 = t3262 * t11498;
    let t11500 = 3.0_f64 / 4.0_f64 * t11499;
    let t11501 = t1065 * t910;
    let t11502 = t3270 * t11501;
    let t11503 = t10667 * t11502;
    let t11504 = 3.0_f64 / 4.0_f64 * t11503;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    (t11495, t11496, t11497, t11498, t11499, t11500, t11502, t11503, t11504, t11505, t11506)
}
