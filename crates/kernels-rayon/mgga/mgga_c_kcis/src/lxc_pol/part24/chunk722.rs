//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 722/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk722(t2354: f64, t88: f64, t66: f64, t673: f64, t2680: f64, t844: f64, t4620: f64, t4714: f64, t8594: f64, t8596: f64, t8598: f64, t8691: f64, t8693: f64, t8695: f64) -> (f64, f64, f64, f64, f64) {
    let t8850 = t88 * t2354;
    let t8858 = t66 * t673;
    let t8862 = t88 * t2680;
    let t8866 = t66 * t844;
    let t8881 = -0.47063e1_f64 * t8594 + 0.31375333333333333334e1_f64 * t8596 - 0.36604555555555555556e1_f64 * t8598 - 0.16068111111111111111e1_f64 * t4620 + 0.28051666666666666666e0_f64 * t8691 - 0.56103333333333333332e0_f64 * t8693 - 0.6545388888888888889e0_f64 * t8695 - 0.46308888888888888888e0_f64 * t4714;
    (t8850, t8858, t8862, t8866, t8881)
}
