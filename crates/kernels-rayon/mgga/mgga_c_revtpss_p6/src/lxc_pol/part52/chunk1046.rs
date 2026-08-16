//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1046/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1046(t32477: f64, t786: f64, t7060: f64, t7063: f64, t31770: f64, t31775: f64, t31835: f64, t31842: f64, t31847: f64, t31855: f64, t32458: f64, t32460: f64, t32463: f64, t32464: f64, t32473: f64, t32476: f64) -> (f64, f64, f64, f64, f64) {
    let t32478 = t786 * t32477;
    let t32480 = 0.14456046980341999104e-1_f64 * t32478 * t7060;
    let t32481 = t7063 * t32477;
    let t32483 = 0.25702851531048074406e-1_f64 * t32481 * t7060;
    let t32485 = -t32458 - 0.3718732920905101082e-3_f64 * t31835 + t32460 - 0.225875734067843736e-2_f64 * t31770 - 0.56468933516960933999e-3_f64 * t31775 - 0.11423947533020470523e1_f64 * t32463 * t32464 + 0.7437465841810202164e-3_f64 * t31842 + 0.14874931683620404328e-2_f64 * t31855 - t32473 + t32476 + t32480 - t32483 + 0.7437465841810202164e-3_f64 * t31847;
    (t32478, t32480, t32481, t32483, t32485)
}
