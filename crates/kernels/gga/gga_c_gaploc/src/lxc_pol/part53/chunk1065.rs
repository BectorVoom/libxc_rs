//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1065/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1065<F: Float>(t44092: F, t44097: F, t44099: F, t44106: F, t44110: F, t44111: F, t44112: F, t44118: F, t47494: F, t47497: F, t47501: F, t47506: F, t47509: F, t47511: F, t47512: F, t47513: F, t47515: F, t47517: F, t47519: F, t47527: F) -> F {
    let t51171 = F::new(0.38342925953920749676e0) * t47494 - F::new(0.21450293971110256002e1) * t47497 - t44092 - t44097 - t44099 - F::new(0.21450293971110256002e1) * t47501 + F::new(0.85206502119823888169e-1) * t47506 + F::new(0.29792074959875355558e-1) * t47509 + t47511 + t47512 - t47513 + t44106 + t47515 + t44110 - t44111 + t44112 + F::new(0.59584149919750711116e-1) * t47517 - F::new(0.21450293971110256002e1) * t47519 - t44118 - F::new(0.13803453343411469884e2) * t47527;
    t51171
}
