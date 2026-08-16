//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 631/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk631(t671: f64, t88: f64, t1268: f64, t1458: f64, t2314: f64, t4026: f64, t4028: f64, t4072: f64, t1390: f64, t1845: f64, t193: f64, t531: f64) -> (f64, f64, f64, f64) {
    let t5113 = t88 * t671;
    let t5118 = 2.0_f64 * t1268 * t4072 + 2.0_f64 * t1458 * t2314 + 2.0_f64 * t1458 * t5113 + 2.0_f64 * t4028 * t671 + t4026;
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    (t5113, t5118, t5122, t5126)
}
