//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 789/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk789(t21314: f64, t469: f64, t6387: f64, t4229: f64, t5885: f64, t2339: f64, t4534: f64, t13900: f64, t2321: f64, t1580: f64, t4374: f64, t442: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21315 = t21314 * sigma0;
    let t21321 = t6387 * t469;
    let t21331 = t5885 * t4229;
    let t21345 = t2339 * t4534;
    let t21620 = t13900 * t2321;
    let t21621 = t1580 * t21620;
    let t21651 = t4374 * t442;
    (t21315, t21321, t21331, t21345, t21621, t21651)
}
