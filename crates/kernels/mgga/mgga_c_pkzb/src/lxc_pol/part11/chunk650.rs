//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 650/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk650<F: Float>(t2303: F, t2308: F, t3017: F, t3059: F, t3732: F, t3744: F, t3748: F, t3752: F, t3754: F, t3759: F, t3763: F) -> (F,) {
    let t3819 = -0.1294625e1 * t3744 + 0.258925e1 * t3748 + t2303 - 0.60385e0 * t3017 + 0.905775e0 * t3732 + 0.82524375e-1 * t3752 + 0.16504875e0 * t3754 + t2308 - 0.33114e0 * t3059 + 0.248355e0 * t3759 + 0.248355e0 * t3763;
    (t3819,)
}
