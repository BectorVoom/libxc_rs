//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1162/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1162(t14687: f64, t14689: f64, t14691: f64, t14693: f64, t14696: f64, t14698: f64, t14701: f64, t14704: f64, t14708: f64, t14710: f64, t14712: f64, t14715: f64, t14719: f64, t14722: f64, t14724: f64, t14727: f64, t14729: f64, t14731: f64, t14733: f64, t14736: f64) -> f64 {
    let t14738 = t14687 / 576.0_f64 - t14689 / 18.0_f64 - 2.0_f64 / 9.0_f64 * t14691 - t14693 / 16.0_f64 + t14696 / 12.0_f64 + 2.0_f64 / 9.0_f64 * t14698 - t14701 / 12.0_f64 + t14704 / 4.0_f64 + t14708 / 288.0_f64 + 11.0_f64 / 18.0_f64 * t14710 - t14712 / 3.0_f64 - t14715 / 96.0_f64 - t14719 / 48.0_f64 + t14722 / 128.0_f64 - 19.0_f64 / 144.0_f64 * t14724 + t14727 / 6.0_f64 + t14729 / 3.0_f64 - t14731 / 72.0_f64 - t14733 / 96.0_f64 + t14736 / 192.0_f64;
    t14738
}
