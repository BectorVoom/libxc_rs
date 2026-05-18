//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1329/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1329<F: Float>(t14729: F, t14731: F, t14732: F, t5414: F, t5417: F, t5419: F, t5422: F, t6022: F, t6025: F, t6604: F, t6607: F, t6612: F, t6616: F) -> F {
    let t24695 = F::new(12.0) * t5414 - F::new(2.0) * t5417 + F::new(6.0) * t6604 + F::new(6.0) * t5419 + F::new(12.0) * t6607 + F::new(6.0) * t5422 - t14729 + t14731 - t14732 + F::new(12.0) * t6612 - F::new(0.11696447245269292414e1) * t6022 - F::new(2.0) * t6616 - F::new(0.36622894612013090108e-3) * t6025;
    t24695
}
