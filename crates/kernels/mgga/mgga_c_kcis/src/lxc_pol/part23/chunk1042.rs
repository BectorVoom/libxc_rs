//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1042/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1042<F: Float>(t27399: F, t303: F, t4007: F, t7914: F, t6176: F, t12286: F, t1598: F) -> (F, F, F, F) {
    let t27400 = t303 * t27399;
    let t27402 = t7914 * t4007;
    let t27403 = t6176 * t27402;
    let t27410 = t12286 * t1598;
    (t27400, t27402, t27403, t27410)
}
