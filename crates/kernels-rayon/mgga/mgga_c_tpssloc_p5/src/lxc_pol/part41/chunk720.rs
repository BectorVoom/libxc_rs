//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 720/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk720(t1128: f64, t1675: f64, t1136: f64, t1683: f64, t3238: f64, t3295: f64, t3339: f64, t3346: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64) -> (f64, f64, f64) {
    let t4797 = t1675 * t1128;
    let t4802 = t1683 * t1136;
    let t4819 = -0.17648625e1_f64 * t4749 + 0.3529725e1_f64 * t4757 + t3339 - 0.17215833333333333333e0_f64 * t3238 - 0.17215833333333333333e0_f64 * t4721 - 0.34431666666666666667e0_f64 * t4726 + 0.103295e1_f64 * t4731 + 0.516475e0_f64 * t4735 + 0.31558125e0_f64 * t4765 + 0.6311625e0_f64 * t4767 + t3346 - 0.69463333333333333333e-1_f64 * t3295 - 0.69463333333333333333e-1_f64 * t4770 - 0.34731666666666666667e-1_f64 * t4773 + 0.20839e0_f64 * t4776 + 0.104195e0_f64 * t4779;
    (t4797, t4802, t4819)
}
