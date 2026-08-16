//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2804/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2804(t40628: f64, t40834: f64, t854: f64, t10890: f64, t2707: f64, t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t10675: f64, t2689: f64) -> (f64, f64, f64, f64, f64) {
    let t40836 = t40834 * t854 * t40628;
    let t40838 = t10890 * t2707;
    let t40846 = t10293 * t240;
    let t40850 = 0.12516778469694349359e-1_f64 * t813 * t40846 * t243 * t816;
    let t40851 = t2689 * t10675;
    (t40836, t40838, t40846, t40850, t40851)
}
