//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2399/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2399(t10293: f64, t240: f64, t243: f64, t813: f64, t816: f64, t10675: f64, t2689: f64, t10777: f64, t10779: f64, t2706: f64, t837: f64, t798: f64, t9726: f64) -> (f64, f64, f64, f64, f64) {
    let t40846 = t10293 * t240;
    let t40850 = 0.12516778469694349359e-1_f64 * t813 * t40846 * t243 * t816;
    let t40851 = t2689 * t10675;
    let t40855 = t10777 * t10779 * t2706 * t837;
    let t40861 = t9726 * t798;
    (t40846, t40850, t40851, t40855, t40861)
}
