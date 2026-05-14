//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1283/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1283<F: Float>(t10947: F, t22568: F, t22571: F, t2302: F, t260: F, t26617: F, t30792: F, t30831: F, t30945: F, t30948: F, t30961: F, t30963: F, t30965: F, t30967: F, t30970: F, t3452: F, t4229: F, t855: F, t9053: F) -> (F,) {
    let t31485 = 0.23392894490538584828e1 * t855 * t3452 * t9053 + 0.19751673498613801407e-1 * t260 * t30792 + t30831 + t30945 + t30948 - t30961 - 0.91082604192152556044e5 * t855 * t22568 * t4229 * t22571 * t2302 - 0.10254018858216406658e4 * t855 * t10947 * t26617 + t30963 - t30965 + t30967 + t30970;
    (t31485,)
}
