//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 735/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk735(t1039: f64, t7741: f64, t7678: f64, t7682: f64, t7686: f64, t7690: f64, t7694: f64, t7697: f64, t7699: f64, t7702: f64, t7706: f64, t7710: f64, t7714: f64, t7715: f64, t7718: f64, t7722: f64, t7726: f64, t7729: f64, t7734: f64, t7738: f64, t7740: f64) -> (f64, f64) {
    let t7742 = t7741 * t1039;
    let t7743 = 0.85748036236139473944e-3_f64 * t7742;
    let t7744 = t7678 - 0.18868855373762491241e-2_f64 * t7682 + 0.40015750243531754508e-2_f64 * t7686 + 0.32155513588552302729e-2_f64 * t7690 + 0.64311027177104605458e-3_f64 * t7694 - t7697 - t7699 - 0.42874018118069736972e-3_f64 * t7702 - 0.21437009059034868486e-3_f64 * t7706 + 0.62896184579208304135e-3_f64 * t7710 - t7714 - 0.42874018118069736972e-3_f64 * t7715 - t7718 - t7722 - t7726 - 0.10718504529517434243e-3_f64 * t7729 + 0.85748036236139473944e-3_f64 * t7734 - t7738 - t7740 + t7743;
    (t7743, t7744)
}
