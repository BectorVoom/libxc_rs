//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 631/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk631(t15392: f64, t15395: f64, t15400: f64, t15406: f64, t15412: f64, t2211: f64, t8975: f64, t739: f64, t8946: f64, t884: f64, t8041: f64, t8936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15664 = 0.35038612185802734376e-6_f64 * t15392;
    let t15665 = 0.1276937996798935182e-4_f64 * t15395;
    let t15666 = 0.72714524817717142308e-5_f64 * t15400;
    let t15667 = 0.85129199786595678799e-5_f64 * t15406;
    let t15668 = 0.58171619854173713846e-5_f64 * t15412;
    let t15669 = t2211 * t8975;
    let t15670 = t739 * t15669;
    let t15671 = 0.11974241701863808564e0_f64 * t15670;
    let t15672 = t2211 * t8946;
    let t15673 = t884 * t15672;
    let t15674 = 0.11974241701863808564e0_f64 * t15673;
    let t15675 = t8041 * t8936;
    (t15664, t15665, t15666, t15667, t15668, t15669, t15671, t15672, t15674, t15675)
}
