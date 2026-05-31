//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 900/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk900<F: Float>(t2970: F, t2974: F, t7848: F, t2971: F, t639: F, t1179: F, t668: F, t545: F, t1796: F, t2973: F, t13: F, t2969: F, t6181: F) -> (F, F, F, F, F, F) {
    let t7851 = t2970 * t7848 * t2974 / F::cast_from(72.0_f64);
    let t7852 = t2971 * t639;
    let t7856 = t668 * t1179;
    let t7857 = t7856 * t545;
    let t7861 = t2973 * t1796;
    let t7866 = t6181 * t13 * t2969;
    (t7851, t7852, t7856, t7857, t7861, t7866)
}
