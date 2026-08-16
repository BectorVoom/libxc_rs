//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1189/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1189(t132: f64, t137: f64, t15544: f64, t15563: f64, t15585: f64, t15611: f64, t15626: f64, t15641: f64, t15662: f64, t15692: f64, t465: f64, t15496: f64, t15498: f64, t15501: f64, t15506: f64, t15509: f64, t15510: f64, t15511: f64, t15516: f64, t15518: f64, t15520: f64, t15522: f64, t15524: f64, t15526: f64, t15527: f64) -> (f64, f64) {
    let t15699 = t132 * t137 * t465 * (t15544 + t15563 + t15585 + t15611 + t15626 + t15641 + t15662 + t15692) / 30.0_f64;
    let t15700 = -t15496 - t15498 - t15501 - t15506 - t15509 + t15510 + t15511 - t15516 + t15518 - t15520 - t15522 + t15524 - t15526 + t15527 - t15699;
    (t15699, t15700)
}
