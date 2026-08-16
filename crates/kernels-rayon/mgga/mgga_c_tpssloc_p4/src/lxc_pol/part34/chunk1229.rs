//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1229/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1229(t105250: f64, t105254: f64, t108164: f64, t108189: f64, t108218: f64, t108321: f64, t1527: f64, t17052: f64, t17092: f64, t2054: f64, t26713: f64, t2718: f64, t29055: f64, t5657: f64, t5658: f64, t67344: f64, t7841: f64, t7842: f64, t84820: f64, t855: f64, t858: f64, t86916: f64) -> f64 {
    let t108342 = -3.0_f64 * t17052 * t7842 - t855 * t858 * (t108164 + t108189 + t108218 + t108321) - t67344 * t2054 + 0.9869604401089358619e-1_f64 * t86916 + t84820 + 6.0_f64 * t855 * t2718 * t29055 * t1527 + 6.0_f64 * t855 * t2718 * t7841 * t5657 - 3.0_f64 * t26713 * t5658 - 6.0_f64 * t17092 * t7842 - 0.16449340668482264365e-1_f64 * t105250 - 0.9869604401089358619e-1_f64 * t105254;
    t108342
}
