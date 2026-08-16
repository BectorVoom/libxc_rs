//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1239/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1239(t771: f64, t7755: f64, t1066: f64, t179: f64, t18107: f64, t299: f64, t2068: f64, t7350: f64, t21603: f64, t5932: f64, t1130: f64, t2104: f64, t2105: f64, t21637: f64, t21640: f64, t21643: f64, t21652: f64, t21655: f64, t2922: f64, t5604: f64, t5913: f64, t5970: f64, t761: f64, t7668: f64, t7695: f64, t7751: f64) -> f64 {
    let t21657 = t771 * t7755;
    let t21658 = 0.15244095330869239812e-2_f64 * t21657;
    let t21661 = t299 * t179 * t18107 * t1066;
    let t21667 = t299 * t179 * t2068 * t7350;
    let t21669 = t5932 * t21603;
    let t21675 = -0.85748036236139473944e-3_f64 * t21637 - 0.17149607247227894789e-2_f64 * t21640 + 0.85748036236139473944e-3_f64 * t21643 - 0.42874018118069736972e-3_f64 * t2104 * t2105 * t1066 * t5913 * t761 - t21652 + 0.10620053080505570402e0_f64 * t5604 * t1130 - 0.28963781128651555642e-1_f64 * t21655 - t21658 - 0.1270341277572436651e-3_f64 * t21661 + 0.68598428988911579154e-2_f64 * t771 * t7751 - 0.85748036236139473944e-3_f64 * t21667 - 0.34299214494455789577e-2_f64 * t21669 * t7668 - 0.38586616306262763276e-2_f64 * t2922 * t7695 * t5970;
    t21675
}
