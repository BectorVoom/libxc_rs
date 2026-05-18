//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1216/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1216<F: Float>(t34237: F, t34255: F, t34263: F, t34271: F, t34273: F, t34283: F, t34286: F, t34288: F, t36994: F, t37008: F, t39112: F, t39114: F, t39118: F, t39122: F, t39131: F, t39134: F, t39136: F) -> F {
    let t41495 = F::new(0.64311027177104605458e-2) * t39112 + F::new(0.64311027177104605458e-2) * t39114 + F::new(0.64311027177104605458e-2) * t39118 + F::new(0.42874018118069736972e-2) * t39122 + F::new(0.85748036236139473944e-3) * t34237 - t36994 - F::new(0.17149607247227894789e-2) * t34255 - F::new(0.25158473831683321656e-2) * t34263 - F::new(0.34299214494455789578e-2) * t34271 - F::new(0.16006300097412701803e-1) * t34273 - t34283 - t37008 + F::new(0.18007087609589289529e-1) * t34286 + t34288 - F::new(0.18868855373762491241e-2) * t39131 + F::new(0.21437009059034868486e-2) * t39134 + F::new(0.34299214494455789578e-2) * t39136;
    t41495
}
