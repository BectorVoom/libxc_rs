//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1214/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1214(t3262: f64, t3263: f64, t40579: f64, t37580: f64, t40536: f64, t40539: f64, t40541: f64, t40544: f64, t40547: f64, t40551: f64, t40554: f64, t40556: f64, t40560: f64, t40564: f64, t40569: f64, t40571: f64, t40578: f64) -> (f64, f64) {
    let t40582 = 3.0_f64 / 4.0_f64 * t3262 * t3263 * t40579;
    let t40583 = t40536 + t40539 - t40541 - t40544 - t40547 - t40551 + t40554 + 0.81300399444200075504e-3_f64 * t40556 + t40560 - 0.43368970657079495312e-4_f64 * t40564 - t40569 - t40571 + 0.68400385060046895006e-6_f64 * t37580 + t40578 + t40582;
    (t40582, t40583)
}
