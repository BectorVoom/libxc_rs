//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1148/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1148(t14494: f64, t6035: f64, t14791: f64, t2703: f64, t5985: f64, t10905: f64, t5989: f64, t10678: f64, t10687: f64, t10692: f64, t14736: f64, t14744: f64, t14759: f64, t14761: f64, t14765: f64, t14777: f64, t2745: f64) -> f64 {
    let t18333 = t14494 * t6035;
    let t18334 = t14791 * t18333;
    let t18338 = t2703 * t5985;
    let t18340 = t10905 * t5989;
    let t18343 = -t14736 + t14744 + t14759 - 0.90357964994909313582e-5_f64 * t14761 - 0.30488190661738479624e-3_f64 * t10678 - t10687 + t10692 + 0.17149607247227894789e-2_f64 * t2745 * t18334 - 35.0_f64 / 108.0_f64 * t14765 + 7.0_f64 / 144.0_f64 * t18338 - 7.0_f64 / 48.0_f64 * t18340 - 0.80031500487063509016e-2_f64 * t14777;
    t18343
}
