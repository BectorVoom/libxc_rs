//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1216/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1216(t34237: f64, t34255: f64, t34263: f64, t34271: f64, t34273: f64, t34283: f64, t34286: f64, t34288: f64, t36994: f64, t37008: f64, t39112: f64, t39114: f64, t39118: f64, t39122: f64, t39131: f64, t39134: f64, t39136: f64) -> f64 {
    let t41495 = 0.64311027177104605458e-2_f64 * t39112 + 0.64311027177104605458e-2_f64 * t39114 + 0.64311027177104605458e-2_f64 * t39118 + 0.42874018118069736972e-2_f64 * t39122 + 0.85748036236139473944e-3_f64 * t34237 - t36994 - 0.17149607247227894789e-2_f64 * t34255 - 0.25158473831683321656e-2_f64 * t34263 - 0.34299214494455789578e-2_f64 * t34271 - 0.16006300097412701803e-1_f64 * t34273 - t34283 - t37008 + 0.18007087609589289529e-1_f64 * t34286 + t34288 - 0.18868855373762491241e-2_f64 * t39131 + 0.21437009059034868486e-2_f64 * t39134 + 0.34299214494455789578e-2_f64 * t39136;
    t41495
}
