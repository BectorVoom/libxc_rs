//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3077/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3077(t448: f64, t56211: f64, t56258: f64, t300: f64, t16784: f64, t3539: f64, t12230: f64, t5104: f64, t12227: f64, t3385: f64, t12357: f64, t3433: f64, t5108: f64) -> (f64, f64, f64, f64, f64) {
    let t56260 = (t56211 + t56258) * t448;
    let t56262 = 0.19751673498613801407e-1_f64 * t300 * t56260;
    let t56264 = 0.17544670867903938621e1_f64 * t16784 * t3539;
    let t56265 = t5104 * t12230;
    let t56268 = 0.1551780387578202009e4_f64 * t12227 * t56265 * t3385;
    let t56271 = 0.16081979498692535067e2_f64 * t3433 * t5108 * t12357;
    (t56260, t56262, t56264, t56268, t56271)
}
