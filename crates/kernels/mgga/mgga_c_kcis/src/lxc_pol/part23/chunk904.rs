//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 904/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk904<F: Float>(t17377: F, t17380: F, t17384: F, t17386: F, t17389: F, t17392: F, t17394: F, t17398: F, t17400: F, t17403: F, t17405: F, t17407: F, t17410: F, t17413: F, t17415: F, t17418: F, t17421: F, t17423: F) -> (F,) {
    let t18311 = -0.10791666666666666667e0 * t17377 + 0.375e0 * t17380 + 0.27777777777777777777e-1 * t17384 + 0.5e0 * t17386 - 0.20234375e-1 * t17389 + 0.26979166666666666666e-1 * t17392 - 0.13489583333333333333e-1 * t17394 + 0.1875e0 * t17398 + 0.26979166666666666666e-1 * t17400 - 0.20833333333333333333e-1 * t17403 + 0.625e-1 * t17405 - 0.125e0 * t17407 - 0.625e-1 * t17410 - 0.125e0 * t17413 + 0.125e0 * t17415 - 0.4046875e-1 * t17418 - 0.5e0 * t17421 - 0.20833333333333333333e-1 * t17423;
    (t18311,)
}
