//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 754/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk754<F: Float>(t8574: F, t8580: F, t8582: F, t7317: F, t7319: F, t8558: F, t8562: F, t8567: F, t8572: F, t8576: F, t8578: F, t8584: F, t8586: F, t8590: F, t9206: F, t8607: F) -> (F, F) {
    let t9211 = 0.85748036236139473944e-3 * t8574;
    let t9214 = 0.18868855373762491241e-2 * t8580;
    let t9215 = 0.21437009059034868486e-3 * t8582;
    let t9219 = t9206 - 0.62896184579208304138e-3 * t8558 - 0.62896184579208304138e-3 * t8562 - 0.62896184579208304138e-3 * t8567 - 0.41930789719472202759e-3 * t8572 - t9211 - 0.85748036236139473944e-3 * t8576 + 0.94344276868812456207e-3 * t8578 - t9214 - t9215 - t8584 / 48.0 - t8586 / 48.0 - t8590 / 64.0 + t7317 + t7319;
    let t9222 = 0.42874018118069736972e-3 * t8607;
    (t9219, t9222)
}
