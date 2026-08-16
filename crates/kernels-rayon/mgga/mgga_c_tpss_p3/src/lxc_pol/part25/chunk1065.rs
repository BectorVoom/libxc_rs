//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1065/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1065(t11004: f64, t11051: f64, t11179: f64, t11188: f64, t14551: f64, t14553: f64, t14556: f64, t14559: f64, t14561: f64, t14564: f64, t8797: f64, t10994: f64, t14454: f64, t14459: f64, t14462: f64, t14466: f64, t14471: f64, t14475: f64, t14479: f64, t14484: f64, t14489: f64, t14492: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t14539: f64, t14541: f64, t14610: f64, t8796: f64) -> f64 {
    let t14630 = -t8797 + 0.19419375e1_f64 * t14551 - 0.258925e1_f64 * t14553 - 0.1294625e1_f64 * t14556 - 0.412621875e-1_f64 * t14559 + 0.16504875e0_f64 * t14561 + 0.82524375e-1_f64 * t14564 - t11179 + 0.36793333333333333333e-1_f64 * t11051 + t11188 - 0.40256666666666666668e0_f64 * t11004;
    let t14632 = -0.82785e-1_f64 * t14454 + 0.12077e1_f64 * t14459 + 0.16557e0_f64 * t14462 - 0.5519e-1_f64 * t14466 - 0.36793333333333333333e-1_f64 * t14471 - 0.49671e0_f64 * t14475 + 0.33114e0_f64 * t14479 + 0.16557e0_f64 * t14484 - 0.27595e-1_f64 * t14489 - 0.301925e0_f64 * t14492 + t14610 - 0.18396666666666666667e0_f64 * t10994 + 0.16504875e0_f64 * t14539 + 0.258925e1_f64 * t14541 - 0.33547222222222222222e0_f64 * t14517 - 0.40256666666666666666e0_f64 * t14521 - 0.181155e1_f64 * t14525 + 0.12077e1_f64 * t14528 - 0.20128333333333333333e0_f64 * t14532 + 0.60385e0_f64 * t14535 - t8796 + t14630;
    t14632
}
