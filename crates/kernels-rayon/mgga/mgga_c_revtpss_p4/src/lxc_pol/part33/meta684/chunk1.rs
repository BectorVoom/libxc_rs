//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2252/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2252(t20842: f64, t7613: f64, t1234: f64, t30815: f64, t20816: f64, t7618: f64, t29020: f64, t5265: f64, t104953: f64, t104963: f64, t104968: f64, t1238: f64, t20792: f64, t21085: f64, t21157: f64, t26867: f64, t7624: f64, t97267: f64, t97272: f64) -> f64 {
    let t112452 = t7613 * t20842;
    let t112456 = t1234 * t30815;
    let t112461 = t7618 * t20816;
    let t112465 = t29020 * t5265;
    let t112467 = -0.28582678745379824648e-3_f64 * t26867 * t21157 - t104953 - 0.28582678745379824648e-3_f64 * t112452 - 0.42874018118069736972e-3_f64 * t7613 * t21085 - 0.14481890564325777821e-1_f64 * t112456 * t1238 + t104963 / 81.0_f64 - 0.95275595817932748827e-4_f64 * t97267 + t97272 + 0.28582678745379824648e-3_f64 * t112461 + 0.47637797908966374413e-3_f64 * t7624 * t20792 - 0.30488190661738479624e-2_f64 * t112465 - t104968;
    t112467
}
