//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1132/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1132<F: Float>(t179: F, t2068: F, t299: F, t7350: F, t21603: F, t5932: F, t1066: F, t1130: F, t2104: F, t2105: F, t21637: F, t21640: F, t21643: F, t21652: F, t21655: F, t21658: F, t21661: F, t2922: F, t5604: F, t5913: F, t5970: F, t761: F, t7668: F, t7695: F, t771: F, t7751: F) -> (F,) {
    let t21667 = t299 * t179 * t2068 * t7350;
    let t21669 = t5932 * t21603;
    let t21675 = -0.85748036236139473944e-3 * t21637 - 0.17149607247227894789e-2 * t21640 + 0.85748036236139473944e-3 * t21643 - 0.42874018118069736972e-3 * t2104 * t2105 * t1066 * t5913 * t761 - t21652 + 0.10620053080505570402e0 * t5604 * t1130 - 0.28963781128651555642e-1 * t21655 - t21658 - 0.1270341277572436651e-3 * t21661 + 0.68598428988911579154e-2 * t771 * t7751 - 0.85748036236139473944e-3 * t21667 - 0.34299214494455789577e-2 * t21669 * t7668 - 0.38586616306262763276e-2 * t2922 * t7695 * t5970;
    (t21675,)
}
