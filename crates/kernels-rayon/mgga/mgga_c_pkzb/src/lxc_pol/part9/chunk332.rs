//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 332/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk332(t1123: f64, t301: f64, t761: f64, t758: f64, t1066: f64, t179: f64, t780: f64, t1120: f64, t276: f64, t299: f64, t741: f64, t757: f64, t777: f64) -> (f64, f64, f64, f64, f64) {
    let t1124 = t301 * t1123;
    let t1125 = t1124 * t761;
    let t1126 = t758 * t1125;
    let t1130 = t179 * t780 * t1066;
    let t1133 = t741 - t276 * t1120 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t757 * t1126 + t777 - 0.42874018118069736972e-3_f64 * t299 * t1130;
    (t1124, t1125, t1126, t1130, t1133)
}
