//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 753/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk753<F: Float>(t8509: F, t8527: F, t8531: F, t8533: F, t8546: F, t8503: F, t8512: F, t8516: F, t8519: F, t8523: F, t8529: F, t8537: F, t8542: F, t8550: F, t9190: F, t8556: F) -> (F, F) {
    let t9191 = 0.15724046144802076034e-2 * t8509;
    let t9196 = 0.14291339372689912324e-3 * t8527;
    let t9198 = 0.12862205435420921092e-2 * t8531;
    let t9199 = 0.10718504529517434243e-2 * t8533;
    let t9202 = 0.7145669686344956162e-3 * t8546;
    let t9204 = -0.42874018118069736972e-3 * t8503 - t9190 + t9191 + 0.17149607247227894789e-2 * t8512 + 0.21437009059034868486e-2 * t8516 - 0.21437009059034868486e-3 * t8519 - 0.21437009059034868486e-3 * t8523 - t9196 + 0.31448092289604152069e-3 * t8529 + t9198 - t9199 - 0.10718504529517434243e-2 * t8537 - 0.10718504529517434243e-2 * t8542 - t9202 + 0.15724046144802076034e-2 * t8550;
    let t9206 = 0.10482697429868050689e-2 * t8556;
    (t9204, t9206)
}
