//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1212/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1212(t3706: f64, t506: f64, t1797: f64, t3573: f64, t20174: f64, t944: f64, t3409: f64, t6245: f64, t1150: f64, t1181: f64, t13364: f64, t16992: f64, t17139: f64, t175: f64, t22102: f64, t22105: f64, t22107: f64, t22112: f64, t22114: f64, t336: f64, t398: f64, t418: f64, t4643: f64, t4735: f64, t5012: f64, t5116: f64, t525: f64, t5630: f64, t922: f64, t942: f64) -> (f64, f64) {
    let t22120 = t3706 * t506;
    let t22125 = t3573 * t1797;
    let t22127 = t20174 * t944;
    let t22132 = t3409 * t6245;
    let t22134 = -0.34299214494455789578e-2_f64 * t418 * t398 * t4643 * t5116 + 0.17149607247227894789e-2_f64 * t22102 - 0.68026775414003982663e-1_f64 * t16992 + 0.34299214494455789578e-2_f64 * t22105 - 0.34299214494455789578e-1_f64 * t17139 * t13364 * t525 * t22107 + 7.0_f64 / 72.0_f64 * t22112 + 7.0_f64 / 72.0_f64 * t22114 + t1150 * t336 * t5630 * t922 / 16.0_f64 - 0.20579528696673473747e-1_f64 * t4735 * t1181 * t22120 * t5012 - 35.0_f64 / 432.0_f64 * t22125 + 0.85748036236139473944e-3_f64 * t942 * t398 * t175 * t22127 - 0.40015750243531754508e-2_f64 * t22132;
    (t22127, t22134)
}
