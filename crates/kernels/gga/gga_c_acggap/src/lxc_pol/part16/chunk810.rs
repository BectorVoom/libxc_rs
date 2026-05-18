//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 810/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk810<F: Float>(t467: F, t560: F, t9097: F, t8453: F, t8459: F, t8494: F, t8507: F, t8509: F, t8527: F, t8531: F, t8533: F, t8546: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9098 = t560 * t467;
    let t9099 = t9097 * t9098;
    let t9176 = F::new(0.85748036236139473944e-3) * t8453;
    let t9178 = F::new(0.15724046144802076034e-2) * t8459;
    let t9186 = F::new(0.42874018118069736972e-3) * t8494;
    let t9190 = F::new(0.28582678745379824648e-3) * t8507;
    let t9191 = F::new(0.15724046144802076034e-2) * t8509;
    let t9196 = F::new(0.14291339372689912324e-3) * t8527;
    let t9198 = F::new(0.12862205435420921092e-2) * t8531;
    let t9199 = F::new(0.10718504529517434243e-2) * t8533;
    let t9202 = F::new(0.7145669686344956162e-3) * t8546;
    (t9098, t9099, t9176, t9178, t9186, t9190, t9191, t9196, t9198, t9199, t9202)
}
