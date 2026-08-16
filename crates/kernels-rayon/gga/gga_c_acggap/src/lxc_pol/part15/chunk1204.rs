//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1204/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1204(t33831: f64, t33853: f64, t33857: f64, t36817: f64, t36818: f64, t36819: f64, t36821: f64, t38701: f64, t38704: f64, t38706: f64, t38709: f64, t38711: f64, t38713: f64, t38717: f64, t38721: f64, t38723: f64, t38727: f64) -> f64 {
    let t41329 = -0.25158473831683321655e-2_f64 * t33831 - 0.85748036236139473944e-3_f64 * t38701 + 0.21437009059034868486e-2_f64 * t38704 - t36817 + 0.17149607247227894789e-2_f64 * t38706 - t36818 + t36819 + t36821 + 0.83861579438944405517e-3_f64 * t33853 + 0.16006300097412701803e-1_f64 * t38709 - 0.18868855373762491241e-2_f64 * t38711 + 0.25158473831683321655e-2_f64 * t33857 + 7.0_f64 / 36.0_f64 * t38713 + 0.37737710747524982483e-2_f64 * t38717 + 0.18868855373762491242e-1_f64 * t38721 - 0.62896184579208304138e-3_f64 * t38723 - 0.41930789719472202759e-3_f64 * t38727;
    t41329
}
