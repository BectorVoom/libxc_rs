//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 798/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk798<F: Float>(t8556: F, t8574: F, t8580: F, t8582: F, t7317: F, t7319: F, t8558: F, t8562: F, t8567: F, t8572: F, t8576: F, t8578: F, t8584: F, t8586: F, t8590: F) -> F {
    let t9206 = F::cast_from(0.10482697429868050689e-2_f64) * t8556;
    let t9211 = F::cast_from(0.85748036236139473944e-3_f64) * t8574;
    let t9214 = F::cast_from(0.18868855373762491241e-2_f64) * t8580;
    let t9215 = F::cast_from(0.21437009059034868486e-3_f64) * t8582;
    let t9219 = t9206 - F::cast_from(0.62896184579208304138e-3_f64) * t8558 - F::cast_from(0.62896184579208304138e-3_f64) * t8562 - F::cast_from(0.62896184579208304138e-3_f64) * t8567 - F::cast_from(0.41930789719472202759e-3_f64) * t8572 - t9211 - F::cast_from(0.85748036236139473944e-3_f64) * t8576 + F::cast_from(0.94344276868812456207e-3_f64) * t8578 - t9214 - t9215 - t8584 / F::new(48.0) - t8586 / F::new(48.0) - t8590 / F::new(64.0) + t7317 + t7319;
    t9219
}
