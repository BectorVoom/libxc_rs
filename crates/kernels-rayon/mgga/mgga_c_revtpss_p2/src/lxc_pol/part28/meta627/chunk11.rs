//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2258/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2258(t2251: f64, t4173: f64, t10301: f64, t28126: f64, t2247: f64, t28076: f64, t38: f64, t28104: f64, t644: f64, t77: f64, t1928: f64, t25102: f64, t25110: f64, t25117: f64, t25157: f64, t28138: f64, t28141: f64, t28147: f64, t6960: f64, t6974: f64, t6978: f64, t7716: f64, t7720: f64, t92684: f64, t92687: f64) -> f64 {
    let t101376 = t4173 * t2251;
    let t101385 = t10301 * t28126;
    let t101391 = t2247 * t38 * t28076;
    let t101399 = t77 * t28104 * t644;
    let t101402 = t25117 * t7716 / 3.0_f64 + t25117 * t7720 / 3.0_f64 + t101376 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28141 * t6974 + 5.0_f64 / 3.0_f64 * t28138 * t25110 + 2.0_f64 / 3.0_f64 * t28141 * t6978 + 5.0_f64 / 3.0_f64 * t101385 * t6960 + 2.0_f64 / 3.0_f64 * t25102 * t7716 + 5.0_f64 / 3.0_f64 * t101391 * t6960 - 10.0_f64 * t92684 * t28147 - 10.0_f64 * t92687 * t28147 - 10.0_f64 * t25157 * t101399;
    t101402
}
