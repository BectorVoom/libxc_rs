//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1073/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1073(t11004: f64, t11051: f64, t11319: f64, t11328: f64, t14551: f64, t14553: f64, t14556: f64, t14559: f64, t14561: f64, t14564: f64, t8872: f64, t10994: f64, t14454: f64, t14459: f64, t14462: f64, t14466: f64, t14471: f64, t14475: f64, t14479: f64, t14484: f64, t14489: f64, t14492: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t14539: f64, t14541: f64, t14770: f64, t8871: f64) -> f64 {
    let t14790 = -t8872 + 0.264729375e1_f64 * t14551 - 0.3529725e1_f64 * t14553 - 0.17648625e1_f64 * t14556 - 0.157790625e0_f64 * t14559 + 0.6311625e0_f64 * t14561 + 0.31558125e0_f64 * t14564 - t11319 + 0.4630888888888888889e-1_f64 * t11051 + t11328 - 0.68863333333333333332e0_f64 * t11004;
    let t14792 = -0.104195e0_f64 * t14454 + 0.20659e1_f64 * t14459 + 0.20839e0_f64 * t14462 - 0.69463333333333333334e-1_f64 * t14466 - 0.46308888888888888889e-1_f64 * t14471 - 0.62517e0_f64 * t14475 + 0.41678e0_f64 * t14479 + 0.20839e0_f64 * t14484 - 0.34731666666666666667e-1_f64 * t14489 - 0.516475e0_f64 * t14492 + t14770 - 0.23154444444444444445e0_f64 * t10994 + 0.6311625e0_f64 * t14539 + 0.3529725e1_f64 * t14541 - 0.57386111111111111112e0_f64 * t14517 - 0.68863333333333333334e0_f64 * t14521 - 0.309885e1_f64 * t14525 + 0.20659e1_f64 * t14528 - 0.34431666666666666667e0_f64 * t14532 + 0.103295e1_f64 * t14535 - t8871 + t14790;
    t14792
}
