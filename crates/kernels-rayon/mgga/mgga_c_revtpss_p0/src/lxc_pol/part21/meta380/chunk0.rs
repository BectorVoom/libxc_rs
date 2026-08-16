//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1792/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1792(t1151: f64, t3427: f64, t3384: f64, t1149: f64, t3435: f64, t3433: f64, t1160: f64, t3444: f64, t1156: f64, t3476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12411 = t1151 * t3427;
    let t12413 = 6.0_f64 * t3384 * t12411;
    let t12415 = t3427 * t3435 * t1149;
    let t12417 = 0.48245938496077605201e2_f64 * t3433 * t12415;
    let t12418 = t3444 * t1160;
    let t12423 = t1156 * t3476;
    (t12411, t12413, t12415, t12417, t12418, t12423)
}
