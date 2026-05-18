//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 38/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk38<F: Float>(t12: F, t15: F) -> (F, F, F, F) {
    let t92 = F::new(0.107924e1) + F::new(0.3964e-1) * t15 + F::new(0.123825e-1) * t12;
    let t95 = F::new(1.0) + t15 * t92 / F::new(2.0);
    let t96 = t95 * t95;
    let t97 = F::new(1.0) / t96;
    (t92, t95, t96, t97)
}
