//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1310/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1310(t13691: f64, t18454: f64, t5373: f64, t60724: f64, t18436: f64, t5377: f64, t18450: f64, t5410: f64, t5383: f64, t60695: f64, t5415: f64, t13795: f64, t5728: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69523 = t18454 * t13691;
    let t69525 = t60724 * t5373;
    let t69527 = t18436 * t5377;
    let t69531 = t18450 * t5410;
    let t69533 = t60695 * t5383;
    let t69535 = t18450 * t5415;
    let t69537 = t5728 * t13795;
    (t69523, t69525, t69527, t69531, t69533, t69535, t69537)
}
