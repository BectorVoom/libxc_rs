//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1312/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1312<F: Float>(t13089: F, t7624: F, t1252: F, t1266: F, t12805: F, t12858: F, t12876: F, t12922: F, t12926: F, t12933: F, t12953: F, t13065: F, t13081: F, t13095: F, t26852: F, t26870: F, t26880: F, t29040: F, t29100: F, t3613: F, t3620: F, t97112: F, t97120: F, t97125: F, t97129: F, t97133: F, t97136: F, t97138: F, t97141: F, t97149: F) -> F {
    let t97154 = t7624 * t13089;
    let t97156 = F::new(0.85748036236139473944e-3) * t97112 - F::new(0.17149607247227894789e-2) * t7624 * t12926 + F::new(0.85748036236139473944e-3) * t26880 * t12933 + F::new(0.17149607247227894789e-2) * t26880 * t12922 + F::new(0.12862205435420921092e-2) * t97120 * t1252 - F::new(0.17149607247227894789e-2) * t29040 * t13081 + F::new(0.17149607247227894789e-2) * t97125 + F::new(0.85748036236139473944e-3) * t26880 * t12953 - F::new(0.12862205435420921092e-2) * t97129 * t3613 + F::new(0.42874018118069736972e-3) * t97133 * t13065 + F::new(0.11433071498151929859e-2) * t97136 - F::new(0.85748036236139473944e-3) * t97138 * t1266 + F::new(0.19055119163586549765e-3) * t97141 + F::new(0.14291339372689912324e-2) * t26852 * t3620 + F::new(0.25724410870841842183e-2) * t29040 * t13095 - F::new(0.12862205435420921092e-2) * t29100 * t12876 - F::new(0.25724410870841842183e-2) * t97149 * t12858 - F::new(0.12862205435420921092e-2) * t26870 * t12805 - F::new(0.11433071498151929859e-2) * t97154;
    t97156
}
