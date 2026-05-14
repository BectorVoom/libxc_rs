//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 200/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk200<F: Float>(t559: F, t609: F, t626: F, t574: F, t586: F) -> (F, F, F) {
    let t629 = t609 * t626 + 0.17411041666666666666e-2 * t559;
    let t632 = 1.0 + 0.9375e-1 * t574 - 0.101171875e-1 * t586;
    let t633 = 1.0 / t632;
    (t629, t632, t633)
}
