//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 623/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk623<F: Float>(t1962: F, t1967: F, t2730: F, t2772: F, t3517: F, t3529: F, t3533: F, t3537: F, t3539: F, t3544: F, t3548: F) -> (F,) {
    let t3604 = -0.1294625e1 * t3529 + 0.258925e1 * t3533 + t1962 - 0.60385e0 * t2730 + 0.905775e0 * t3517 + 0.82524375e-1 * t3537 + 0.16504875e0 * t3539 + t1967 - 0.33114e0 * t2772 + 0.248355e0 * t3544 + 0.248355e0 * t3548;
    (t3604,)
}
