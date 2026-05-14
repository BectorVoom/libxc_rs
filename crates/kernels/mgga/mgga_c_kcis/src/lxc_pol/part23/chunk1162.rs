//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1162/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1162<F: Float>(t3801: F, t6159: F, t6207: F, t27567: F, t99422: F, t27569: F, t27583: F, t7978: F, t98709: F, t98712: F, t98715: F, t98719: F, t98725: F, t99087: F, t99166: F, t99432: F, t99565: F) -> (F, F) {
    let t99569 = t6159 * t6207 * t3801;
    let t99578 = 0.10306077835648148148e-4 * t27567 * t99422;
    let t99585 = 0.30918233506944444444e-4 * t99565 * t27569 - 0.30918233506944444444e-4 * t27567 * t99569 - 0.17411041666666666666e-2 * t98709 + 0.23214722222222222222e-2 * t98712 + 0.11607361111111111111e-2 * t98715 - 0.38691203703703703703e-3 * t98719 + 0.11607361111111111111e-2 * t98725 + t99578 + 0.13901041666666666667e-2 * t27583 * t99432 - 0.13901041666666666667e-2 * t7978 * t99087 - 0.34752604166666666667e-3 * t7978 * t99166;
    (t99569, t99585)
}
