//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1115/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1115<F: Float>(t43569: F, t43571: F, t43575: F, t43579: F, t43582: F, t43588: F, t43592: F, t43597: F, t43601: F, t43602: F, t43603: F, t43604: F) -> F {
    let t47249 = t43569 + t43571 - t43575 + t43579 - t43582 - F::new(0.71500979903700853338e0) * t43588 + t43592 - t43597 + t43601 + t43602 - t43603 - t43604;
    t47249
}
