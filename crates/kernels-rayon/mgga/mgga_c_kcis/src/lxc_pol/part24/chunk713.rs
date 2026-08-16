//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 713/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk713(t662: f64, t8734: f64, t646: f64, t2337: f64, t644: f64, t14: f64, t2340: f64, t31: f64, t8663: f64, t666: f64, t671: f64, t8674: f64, t8678: f64, t8682: f64, t8700: f64, t8704: f64, t8708: f64, t8713: f64, t8717: f64, t8725: f64) -> (f64, f64, f64) {
    let t8735 = t8734 * t662;
    let t8737 = 1.0_f64 * t646 * t8735;
    let t8739 = 1.0_f64 / t2337 / t644;
    let t8740 = t14 * t8739;
    let t8742 = 1.0_f64 / t2340 / t31;
    let t8743 = t8663 * t8742;
    let t8745 = 0.51725014705706168417e3_f64 * t8740 * t8743;
    let t8746 = t8674 + t8678 + 0.1038945353962551798e3_f64 * t671 * t8682 - 0.58482233974552040708e0_f64 * t671 * t8700 - 0.35089340384731224426e1_f64 * t671 * t8704 + 0.35089340384731224426e1_f64 * t671 * t8708 - 0.51947267698127589897e2_f64 * t671 * t8713 - 0.56969282336565386482e-3_f64 * t666 * t8717 - t8725 + t8737 + t8745;
    (t8737, t8745, t8746)
}
