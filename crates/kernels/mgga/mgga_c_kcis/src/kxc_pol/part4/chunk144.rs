//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 144/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk144<F: Float>(t362: F, t413: F, t430: F, t378: F, t390: F) -> (F, F, F) {
    let t433 = t413 * t430 + F::new(0.17411041666666666666e-2) * t362;
    let t436 = F::new(1.0) + F::new(0.9375e-1) * t378 - F::new(0.101171875e-1) * t390;
    let t437 = F::new(1.0) / t436;
    (t433, t436, t437)
}
