//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 762/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk762<F: Float>(t598: F, t8536: F, t1298: F, t137: F, t1426: F, t368: F, t1479: F, t7476: F, t1980: F, t1095: F, t8503: F, t8507: F, t8509: F, t8512: F, t8516: F, t8519: F, t8523: F, t8527: F, t8529: F, t8531: F, t8533: F) -> (F, F, F, F, F, F, F) {
    let t8537 = t598 * t8536;
    let t8539 = t137 * t1298;
    let t8541 = t1426 * t368 * t8539;
    let t8542 = t598 * t8541;
    let t8544 = t368 * t1479;
    let t8545 = t7476 * t8544;
    let t8546 = t1980 * t8545;
    let t8549 = t1426 * t1095 * t8539;
    let t8550 = t598 * t8549;
    let t8552 = -F::new(0.21437009059034868486e-3) * t8503 - F::new(0.14291339372689912324e-3) * t8507 + F::new(0.7862023072401038017e-3) * t8509 + F::new(0.85748036236139473944e-3) * t8512 + F::new(0.10718504529517434243e-2) * t8516 - F::new(0.10718504529517434243e-3) * t8519 - F::new(0.10718504529517434243e-3) * t8523 - F::new(0.7145669686344956162e-4) * t8527 + F::new(0.15724046144802076034e-3) * t8529 + F::new(0.64311027177104605458e-3) * t8531 - F::new(0.53592522647587171215e-3) * t8533 - F::new(0.53592522647587171215e-3) * t8537 - F::new(0.53592522647587171215e-3) * t8542 - F::new(0.3572834843172478081e-3) * t8546 + F::new(0.7862023072401038017e-3) * t8550;
    (t8539, t8541, t8544, t8545, t8546, t8549, t8552)
}
