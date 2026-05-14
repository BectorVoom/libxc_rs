//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1009/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1009<F: Float>(t187: F, t29025: F, t29027: F, t29029: F, t29030: F, t29031: F, t29033: F, t29035: F, t29038: F, t29041: F, t29044: F, t29082: F, t29092: F, t29216: F, t236: F, t233: F) -> (F, F, F) {
    let t29219 = t29025 - t29027 + t29029 - t29030 - t29031 + t29033 - t29035 - t29038 + t29041 + t29044 - t29082 + t187 * (t29092 + t29216);
    let t29220 = t236 * t29219;
    let t29221 = t233 * t29220;
    (t29219, t29220, t29221)
}
