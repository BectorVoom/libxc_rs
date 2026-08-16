//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 617/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk617(t219: f64, t3641: f64, t3648: f64, t222: f64, t73: f64, t1364: f64, t799: f64, t750: f64, t3610: f64, t778: f64, t1373: f64, t1375: f64, t224: f64, t776: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3650 = (t3641 + t3648) * t219;
    let t3656 = t222 * t73;
    let t3657 = t799 * t1364;
    let t3658 = t3657 * t750;
    let t3661 = t778 * t3610;
    let t3664 = 3.0_f64 * t1373 * t779 + 3.0_f64 * t1375 * t776 + 3.0_f64 * t222 * t3661 - t224 * t3650 - 12.0_f64 * t3656 * t3658;
    (t3650, t3656, t3657, t3658, t3661, t3664)
}
