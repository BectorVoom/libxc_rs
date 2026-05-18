//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 342/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk342<F: Float>(t1167: F, t1173: F, t1177: F, t1181: F, t1185: F, t1190: F, t1197: F, t1201: F) -> F {
    let t1291 = F::new(0.9375e-1) * t1167 - F::new(0.9375e-1) * t1173 - F::new(0.25e0) * t1177 + F::new(0.625e-1) * t1181 - F::new(0.101171875e-1) * t1185 + F::new(0.101171875e-1) * t1190 + F::new(0.53958333333333333333e-1) * t1197 - F::new(0.13489583333333333333e-1) * t1201;
    t1291
}
