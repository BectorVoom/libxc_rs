//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 729/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk729(t421: f64, t3356: f64, t1156: f64, t1160: f64, t1159: f64, t431: f64, t426: f64, t3413: f64, t434: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3434 = t421 * t421;
    let t3435 = 1.0_f64 / t3434;
    let t3439 = 0.22831111111111111111e-1_f64 * t3356;
    let t3447 = t1156 * t1160;
    let t3450 = t1159 * t431;
    let t3451 = 1.0_f64 / t3450;
    let t3452 = t426 * t3451;
    let t3459 = 0.68863333333333333333e0_f64 * t3356;
    let t3466 = 0.17365833333333333333e0_f64 * t3413;
    let t3475 = t1159 * t1159;
    let t3476 = 1.0_f64 / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    (t3434, t3435, t3439, t3447, t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478)
}
