//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1033/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1033(t1864: f64, t3668: f64, t14687: f64, t14689: f64, t14691: f64, t14693: f64, t14696: f64, t14698: f64, t14701: f64, t14704: f64, t14708: f64, t14710: f64, t14712: f64, t14715: f64, t14719: f64, t14722: f64, t14724: f64, t14727: f64, t14729: f64, t14731: f64, t14733: f64, t14736: f64) -> (f64, f64) {
    let t15692 = t1864 * t3668;
    let t15716 = 0.44965277777777777777e-2_f64 * t14687 - 0.14388888888888888889e0_f64 * t14689 - 0.33333333333333333334e0_f64 * t14691 - 0.9375e-1_f64 * t14693 + 0.125e0_f64 * t14696 + 0.33333333333333333334e0_f64 * t14698 - 0.125e0_f64 * t14701 + 0.375e0_f64 * t14704 + 0.89930555555555555554e-2_f64 * t14708 + 0.91666666666666666667e0_f64 * t14710 - 0.5e0_f64 * t14712 - 0.26979166666666666666e-1_f64 * t14715 - 0.53958333333333333333e-1_f64 * t14719 + 0.20234375e-1_f64 * t14722 - 0.34173611111111111111e0_f64 * t14724 + 0.25e0_f64 * t14727 + 0.5e0_f64 * t14729 - 0.20833333333333333333e-1_f64 * t14731 - 0.26979166666666666666e-1_f64 * t14733 + 0.13489583333333333333e-1_f64 * t14736;
    (t15692, t15716)
}
