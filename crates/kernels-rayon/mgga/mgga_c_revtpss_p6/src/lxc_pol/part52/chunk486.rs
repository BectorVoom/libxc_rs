//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 486/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk486(t3434: f64, t3356: f64, t1156: f64, t1160: f64, t1159: f64, t431: f64, t426: f64, t3413: f64, t434: f64, t1175: f64, t1179: f64, t1178: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
    let t3479 = 1.0_f64 / t3478;
    let t3483 = 0.12361111111111111111e-1_f64 * t3356;
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    (t3435, t3439, t3447, t3452, t3459, t3466, t3477, t3479, t3483, t3491, t3494)
}
