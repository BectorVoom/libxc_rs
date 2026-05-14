//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 881/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk881<F: Float>(t1046: F, t2776: F, t1041: F, t2808: F, t2785: F, t16: F, t3038: F, t488: F, t1094: F, t2848: F, t517: F) -> (F, F, F, F, F, F, F) {
    let t7783 = 12.0 * t1046 * t2776;
    let t7784 = t1041 * t2808;
    let t7786 = t1046 * t2808;
    let t7788 = t1041 * t2785;
    let t7791 = t16 * t3038 * t488;
    let t7793 = 0.56968947174242584612e-3 * t1094 * t7791;
    let t7817 = t2848 * t517;
    (t7783, t7784, t7786, t7788, t7791, t7793, t7817)
}
