//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1125/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1125<F: Float>(t19377: F, t19378: F, t19381: F, t19384: F, t19387: F, t19390: F, t19393: F, t19396: F, t19397: F, t19400: F, t19403: F, t19410: F, t434: F, t4784: F, t4812: F, t4820: F, t6658: F, t6659: F, t6665: F, t6679: F, t7: F, t974: F, t980: F) -> F {
    let t19417 = -F::new(10.0) / F::new(9.0) * t19377 * t19378 + F::new(10.0) / F::new(9.0) * t19377 * t19381 - F::new(10.0) / F::new(3.0) * t6679 * t19384 - F::new(10.0) * t6658 * t19387 + F::new(10.0) * t6679 * t19390 - F::new(160.0) / F::new(9.0) * t19393 * t6659 - F::new(10.0) / F::new(9.0) * t19396 * t19397 - F::new(10.0) / F::new(9.0) * t19396 * t19400 + F::new(10.0) / F::new(3.0) * t6658 * t19403 - F::new(6160.0) / F::new(81.0) * t4784 * t974 - F::new(40.0) / F::new(3.0) * t434 * t6665 - F::new(10.0) * t7 * t19410 - F::new(40.0) / F::new(9.0) * t980 * t4820 + F::new(80.0) / F::new(81.0) * t980 * t4812;
    t19417
}
