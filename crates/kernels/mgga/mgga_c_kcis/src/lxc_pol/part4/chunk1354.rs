//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1354/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1354<F: Float>(t17377: F, t17380: F, t17384: F, t17386: F, t17389: F, t17392: F, t17394: F, t17398: F, t17400: F, t17403: F, t17405: F, t17407: F, t17410: F, t17413: F, t17415: F, t17418: F, t17421: F, t17423: F) -> F {
    let t17425 = -t17377 / F::new(24.0) + t17380 / F::new(4.0) + t17384 / F::new(54.0) + t17386 / F::new(3.0) - t17389 / F::new(128.0) + t17392 / F::new(96.0) - t17394 / F::new(192.0) + t17398 / F::new(8.0) + t17400 / F::new(96.0) - t17403 / F::new(72.0) + t17405 / F::new(24.0) - t17407 / F::new(12.0) - t17410 / F::new(24.0) - t17413 / F::new(12.0) + t17415 / F::new(12.0) - t17418 / F::new(64.0) - t17421 / F::new(3.0) - t17423 / F::new(72.0);
    t17425
}
