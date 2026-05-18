//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1204/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1204<F: Float>(t33831: F, t33853: F, t33857: F, t36817: F, t36818: F, t36819: F, t36821: F, t38701: F, t38704: F, t38706: F, t38709: F, t38711: F, t38713: F, t38717: F, t38721: F, t38723: F, t38727: F) -> F {
    let t41329 = -F::new(0.25158473831683321655e-2) * t33831 - F::new(0.85748036236139473944e-3) * t38701 + F::new(0.21437009059034868486e-2) * t38704 - t36817 + F::new(0.17149607247227894789e-2) * t38706 - t36818 + t36819 + t36821 + F::new(0.83861579438944405517e-3) * t33853 + F::new(0.16006300097412701803e-1) * t38709 - F::new(0.18868855373762491241e-2) * t38711 + F::new(0.25158473831683321655e-2) * t33857 + F::new(7.0) / F::new(36.0) * t38713 + F::new(0.37737710747524982483e-2) * t38717 + F::new(0.18868855373762491242e-1) * t38721 - F::new(0.62896184579208304138e-3) * t38723 - F::new(0.41930789719472202759e-3) * t38727;
    t41329
}
