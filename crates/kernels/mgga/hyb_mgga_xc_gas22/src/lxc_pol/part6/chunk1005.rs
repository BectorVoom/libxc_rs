//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1005/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1005<F: Float>(t2973: F, t3: F, t3917: F, t668: F, t26: F, t1232: F, t2950: F, t1181: F, t19: F, t2949: F, t2970: F, t2972: F, t2974: F, t3115: F, t3119: F, t7835: F, t7842: F, t7851: F, t7866: F, t7868: F, t9825: F, t9827: F, t9829: F, t9834: F, t9839: F) -> (F, F, F, F, F) {
    let t9846 = t2973 * t3;
    let t9850 = t3917 * t668;
    let t9851 = t26 * t9850;
    let t9858 = t2950 * t1232;
    let t9861 = -t9825 / 64.0 - t9827 / 32.0 - t7851 - t2970 * t9829 * t2974 / 24.0 - t2970 * t2972 * t9834 / 48.0 + t7842 * t2972 * t9839 / 16.0 - 7.0 / 144.0 * t7866 * t7868 * t9839 - t2970 * t7835 * t9846 / 12.0 - 3.0 / 64.0 * t19 * t9851 - 3.0 / 32.0 * t1181 * t3115 - 3.0 / 32.0 * t1181 * t3119 - 3.0 / 16.0 * t2949 * t9858;
    (t9846, t9850, t9851, t9858, t9861)
}
