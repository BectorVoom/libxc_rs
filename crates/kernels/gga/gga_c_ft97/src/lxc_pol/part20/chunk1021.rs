//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1021/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1021<F: Float>(t6101: F, t8232: F, t1882: F, t24687: F, t24683: F, t6156: F, t24737: F, t53891: F, t24201: F, t24237: F, t24395: F, t263: F, t458: F, t5995: F, t6005: F, t24242: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98061 = t8232 * t6101;
    let t98063 = t1882 * t24687;
    let t98065 = t1882 * t24683;
    let t98078 = t8232 * t6156;
    let t98123 = t53891 * t24737;
    let t98139 = t24237 * t24201;
    let t98143 = t24395 * t263;
    let t98152 = t5995 * t458;
    let t98153 = t98152 * t6005;
    let t98157 = t24237 * t24242;
    (t98061, t98063, t98065, t98078, t98123, t98139, t98143, t98152, t98153, t98157)
}
