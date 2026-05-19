//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1239/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1239<F: Float>(t771: F, t7755: F, t1066: F, t179: F, t18107: F, t299: F, t2068: F, t7350: F, t21603: F, t5932: F, t1130: F, t2104: F, t2105: F, t21637: F, t21640: F, t21643: F, t21652: F, t21655: F, t2922: F, t5604: F, t5913: F, t5970: F, t761: F, t7668: F, t7695: F, t7751: F) -> F {
    let t21657 = t771 * t7755;
    let t21658 = F::cast_from(0.15244095330869239812e-2_f64) * t21657;
    let t21661 = t299 * t179 * t18107 * t1066;
    let t21667 = t299 * t179 * t2068 * t7350;
    let t21669 = t5932 * t21603;
    let t21675 = -F::cast_from(0.85748036236139473944e-3_f64) * t21637 - F::cast_from(0.17149607247227894789e-2_f64) * t21640 + F::cast_from(0.85748036236139473944e-3_f64) * t21643 - F::cast_from(0.42874018118069736972e-3_f64) * t2104 * t2105 * t1066 * t5913 * t761 - t21652 + F::cast_from(0.10620053080505570402e0_f64) * t5604 * t1130 - F::cast_from(0.28963781128651555642e-1_f64) * t21655 - t21658 - F::cast_from(0.1270341277572436651e-3_f64) * t21661 + F::cast_from(0.68598428988911579154e-2_f64) * t771 * t7751 - F::cast_from(0.85748036236139473944e-3_f64) * t21667 - F::cast_from(0.34299214494455789577e-2_f64) * t21669 * t7668 - F::cast_from(0.38586616306262763276e-2_f64) * t2922 * t7695 * t5970;
    t21675
}
