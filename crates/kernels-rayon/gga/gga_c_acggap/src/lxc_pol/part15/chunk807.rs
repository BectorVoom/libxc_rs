//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 807/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk807(t8841: f64, t8847: f64, t8851: f64, t8860: f64, t8233: f64, t8835: f64, t8839: f64, t8843: f64, t8845: f64, t8849: f64, t8856: f64, t8862: f64, t8864: f64, t8866: f64, t8870: f64) -> f64 {
    let t9313 = 0.17149607247227894789e-2_f64 * t8841;
    let t9316 = 0.17149607247227894789e-2_f64 * t8847;
    let t9318 = 0.21437009059034868486e-3_f64 * t8851;
    let t9320 = 0.14291339372689912324e-3_f64 * t8860;
    let t9325 = -t8233 + 0.40015750243531754507e-2_f64 * t8835 - 0.10718504529517434243e-2_f64 * t8839 + t9313 + 0.17149607247227894789e-2_f64 * t8843 + 0.17149607247227894789e-2_f64 * t8845 - t9316 - 0.17149607247227894789e-2_f64 * t8849 + t9318 + 0.21437009059034868486e-3_f64 * t8856 + t9320 + 0.37737710747524982483e-2_f64 * t8862 - t8864 / 48.0_f64 - t8866 / 24.0_f64 + 0.31448092289604152069e-3_f64 * t8870;
    t9325
}
