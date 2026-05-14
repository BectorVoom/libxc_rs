//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 166/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk166<F: Float>(t453: F, t456: F) -> (F, F, F, F) {
    let t513 = 0.107924e1 + 0.3964e-1 * t456 + 0.123825e-1 * t453;
    let t516 = 1.0 + t456 * t513 / 2.0;
    let t517 = t516 * t516;
    let t518 = 1.0 / t517;
    (t513, t516, t517, t518)
}
