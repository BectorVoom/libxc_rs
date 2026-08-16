//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1207/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1207(t114: f64, t1270: f64, t3234: f64, t1799: f64, t3166: f64, t18392: f64, t18395: f64, t18398: f64, t18400: f64) -> (f64, f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t18551 = t1270 * t3234;
    let t18613 = t3166 * t1799;
    let t18622 = 22.0_f64 / 9.0_f64 * t18392;
    let t18627 = piecewise3(t115, 0.0_f64, t18622 + 4.0_f64 / 3.0_f64 * t18395 + t18398 / 2.0_f64 - t18400 / 4.0_f64);
    (t18551, t18613, t18622, t18627)
}
