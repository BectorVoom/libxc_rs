//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1160/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1160(t1176: f64, t5165: f64, t13265: f64, t3438: f64, t5175: f64, t1121: f64, t4823: f64, t13105: f64, t381: f64, t1189: f64, t1809: f64, t3355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14712 = t5165 * t1176;
    let t14714 = t3438 * t13265;
    let t14715 = t5175 * t14714;
    let t14717 = t4823 * t1121;
    let t14718 = t3438 * t14717;
    let t14719 = t5175 * t14718;
    let t14721 = t13105 * t381;
    let t14722 = t14721 * t1189;
    let t14724 = t1809 * t3355;
    (t14712, t14715, t14717, t14719, t14722, t14724)
}
