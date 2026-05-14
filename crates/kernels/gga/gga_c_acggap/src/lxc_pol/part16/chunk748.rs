//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 748/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk748<F: Float>(t467: F, t560: F, t9097: F, t8453: F, t8459: F, t8494: F, t8507: F, t8509: F, t8527: F, t8531: F, t8533: F, t8546: F, t8556: F, t8574: F, t8580: F, t8582: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9098 = t560 * t467;
    let t9099 = t9097 * t9098;
    let t9176 = 0.85748036236139473944e-3 * t8453;
    let t9178 = 0.15724046144802076034e-2 * t8459;
    let t9186 = 0.42874018118069736972e-3 * t8494;
    let t9190 = 0.28582678745379824648e-3 * t8507;
    let t9191 = 0.15724046144802076034e-2 * t8509;
    let t9196 = 0.14291339372689912324e-3 * t8527;
    let t9198 = 0.12862205435420921092e-2 * t8531;
    let t9199 = 0.10718504529517434243e-2 * t8533;
    let t9202 = 0.7145669686344956162e-3 * t8546;
    let t9206 = 0.10482697429868050689e-2 * t8556;
    let t9211 = 0.85748036236139473944e-3 * t8574;
    let t9214 = 0.18868855373762491241e-2 * t8580;
    let t9215 = 0.21437009059034868486e-3 * t8582;
    (t9098, t9099, t9176, t9178, t9186, t9190, t9191, t9196, t9198, t9199, t9202, t9206, t9211, t9214, t9215)
}
