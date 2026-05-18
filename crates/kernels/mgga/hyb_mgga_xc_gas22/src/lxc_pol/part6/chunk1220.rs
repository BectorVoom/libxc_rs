//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1220/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1220<F: Float>(t2014: F, t684: F, t8566: F, t2024: F, t6479: F, t8570: F, t3283: F, t6469: F, t214: F, t8438: F, t2026: F, t2986: F) -> (F, F, F, F, F) {
    let t23788 = t684 * t2014 * t8566;
    let t23791 = t2024 * t6479 * t8570;
    let t23802 = t684 * t6469 * t3283;
    let t23804 = t8438 * t214;
    let t23809 = t2986 * t2026;
    (t23788, t23791, t23802, t23804, t23809)
}
