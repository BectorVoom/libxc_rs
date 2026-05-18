//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1164/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1164<F: Float>(t34127: F, t34130: F, t34156: F, t34158: F, t30301: F, t34123: F, t34132: F, t34135: F, t34138: F, t34142: F, t34146: F, t34148: F, t34150: F, t34152: F, t34154: F, t34162: F, t34166: F) -> F {
    let t36950 = F::new(0.28582678745379824648e-3) * t34127;
    let t36951 = F::new(0.42874018118069736972e-3) * t34130;
    let t36961 = F::new(0.18868855373762491241e-2) * t34156;
    let t36962 = F::new(0.13719685797782315831e-1) * t34158;
    let t36966 = -F::new(0.42874018118069736972e-3) * t34123 + t36950 + t36951 - F::new(0.75475421495049964965e-2) * t34132 + F::new(0.21437009059034868486e-2) * t34135 + F::new(0.85748036236139473944e-3) * t34138 + F::new(0.31448092289604152069e-2) * t34142 + F::new(0.62896184579208304138e-2) * t34146 + F::new(0.68598428988911579156e-2) * t34148 - F::new(0.34299214494455789578e-2) * t34150 + F::new(0.34299214494455789578e-2) * t34152 - F::new(0.17149607247227894789e-2) * t34154 - t36961 - t36962 - F::new(0.7717323261252552655e-1) * t34162 + F::new(0.64311027177104605458e-2) * t34166 + F::new(0.40015750243531754507e-2) * t30301;
    t36966
}
