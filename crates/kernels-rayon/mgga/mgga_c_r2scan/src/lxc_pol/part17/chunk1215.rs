//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1215/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1215(t3262: f64, t3472: f64, t42855: f64, t11336: f64, t37327: f64, t42886: f64, t12929: f64, t2449: f64, t3787: f64, t44108: f64, t44110: f64, t44113: f64, t44115: f64, t44117: f64, t44120: f64, t44122: f64, t44125: f64, t44127: f64, t44129: f64, t44132: f64, t44135: f64, t885: f64) -> (f64, f64, f64) {
    let t44140 = 15.0_f64 / 16.0_f64 * t3262 * t3472 * t42855;
    let t44143 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t42886;
    let t44144 = t12929 * t885 + 2.0_f64 * t2449 * t3787 - t44108 + t44110 + t44113 - t44115 + t44117 + t44120 - t44122 - t44125 + t44127 + t44129 + t44132 - t44135 - t44140 + t44143;
    (t44140, t44143, t44144)
}
