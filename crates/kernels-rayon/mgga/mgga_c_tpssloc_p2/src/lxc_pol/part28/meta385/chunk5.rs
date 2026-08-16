//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1497/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1497(t15700: f64, t15702: f64, t3578: f64, t1215: f64, t607: f64, t475: f64, t4728: f64, t1735: f64, t3243: f64, t11668: f64, t1744: f64, t3540: f64) -> (f64, f64, f64, f64, f64) {
    let t15703 = t15700 * t15702;
    let t15704 = t3578 * t15703;
    let t15707 = t607 * t1215;
    let t15708 = t15707 * t475;
    let t15709 = t4728 * t15708;
    let t15710 = t3578 * t15709;
    let t15713 = t1735 * t3243;
    let t15714 = t11668 * t15713;
    let t15717 = t1744 * t3540;
    (t15704, t15708, t15710, t15714, t15717)
}
