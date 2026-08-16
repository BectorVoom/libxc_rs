//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1009/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1009(t5366: f64, t541: f64, t10019: f64, t10028: f64, t1196: f64, t1206: f64, t12673: f64, t1268: f64, t1270: f64, t12757: f64, t12769: f64, t12780: f64, t13627: f64, t13631: f64, t13637: f64, t13641: f64, t13645: f64, t13671: f64, t13808: f64, t13810: f64, t13943: f64, t1625: f64, t198: f64, t3183: f64, t4397: f64, t4478: f64, t4524: f64, t4528: f64, t4532: f64, t509: f64, t9972: f64, t9980: f64) -> f64 {
    let t13950 = t541 * t5366;
    let t13954 = -t12757 + 2.0_f64 * t4524 * t13627 * t1268 + t13631 - t12769 - t9972 + 12.0_f64 * t4532 * t4528 * t4478 - t13637 + 6.0_f64 * t3183 * t12673 * t1625 - 3.0_f64 * t3183 * t13641 * t1206 - t9980 + t13645 + t10019 + t12780 + 3.0_f64 * t198 * t1196 * t13671 + t198 * t509 * t13943 * t1270 - t10028 - t13808 + 6.0_f64 * t3183 * t4528 * t4397 - t13810 + 6.0_f64 * t4532 * t13950 * t1206;
    t13954
}
