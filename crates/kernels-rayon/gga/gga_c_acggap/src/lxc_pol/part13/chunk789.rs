//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 789/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk789(t1426: f64, t368: f64, t8539: f64, t598: f64, t1479: f64, t7476: f64, t1980: f64, t1095: f64, t8503: f64, t8507: f64, t8509: f64, t8512: f64, t8516: f64, t8519: f64, t8523: f64, t8527: f64, t8529: f64, t8531: f64, t8533: f64, t8537: f64) -> (f64, f64, f64, f64, f64) {
    let t8541 = t1426 * t368 * t8539;
    let t8542 = t598 * t8541;
    let t8544 = t368 * t1479;
    let t8545 = t7476 * t8544;
    let t8546 = t1980 * t8545;
    let t8549 = t1426 * t1095 * t8539;
    let t8550 = t598 * t8549;
    let t8552 = -0.21437009059034868486e-3_f64 * t8503 - 0.14291339372689912324e-3_f64 * t8507 + 0.7862023072401038017e-3_f64 * t8509 + 0.85748036236139473944e-3_f64 * t8512 + 0.10718504529517434243e-2_f64 * t8516 - 0.10718504529517434243e-3_f64 * t8519 - 0.10718504529517434243e-3_f64 * t8523 - 0.7145669686344956162e-4_f64 * t8527 + 0.15724046144802076034e-3_f64 * t8529 + 0.64311027177104605458e-3_f64 * t8531 - 0.53592522647587171215e-3_f64 * t8533 - 0.53592522647587171215e-3_f64 * t8537 - 0.53592522647587171215e-3_f64 * t8542 - 0.3572834843172478081e-3_f64 * t8546 + 0.7862023072401038017e-3_f64 * t8550;
    (t8541, t8544, t8545, t8549, t8552)
}
