//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 797/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk797(t8507: f64, t8509: f64, t8527: f64, t8531: f64, t8533: f64, t8546: f64, t8503: f64, t8512: f64, t8516: f64, t8519: f64, t8523: f64, t8529: f64, t8537: f64, t8542: f64, t8550: f64) -> f64 {
    let t9190 = 0.28582678745379824648e-3_f64 * t8507;
    let t9191 = 0.15724046144802076034e-2_f64 * t8509;
    let t9196 = 0.14291339372689912324e-3_f64 * t8527;
    let t9198 = 0.12862205435420921092e-2_f64 * t8531;
    let t9199 = 0.10718504529517434243e-2_f64 * t8533;
    let t9202 = 0.7145669686344956162e-3_f64 * t8546;
    let t9204 = -0.42874018118069736972e-3_f64 * t8503 - t9190 + t9191 + 0.17149607247227894789e-2_f64 * t8512 + 0.21437009059034868486e-2_f64 * t8516 - 0.21437009059034868486e-3_f64 * t8519 - 0.21437009059034868486e-3_f64 * t8523 - t9196 + 0.31448092289604152069e-3_f64 * t8529 + t9198 - t9199 - 0.10718504529517434243e-2_f64 * t8537 - 0.10718504529517434243e-2_f64 * t8542 - t9202 + 0.15724046144802076034e-2_f64 * t8550;
    t9204
}
