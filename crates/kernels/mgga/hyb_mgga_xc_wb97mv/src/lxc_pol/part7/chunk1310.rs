//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1310/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1310<F: Float>(t23174: F, t23178: F, t23180: F, t31771: F, t31812: F, t31817: F, t31820: F, t31823: F, t31827: F, t31831: F, t31835: F, t23132: F, t23135: F, t23172: F, t23183: F, t27153: F, t27156: F, t27159: F, t31841: F, t31844: F, t31851: F, t31853: F, t31856: F) -> (F, F) {
    let t31994 = -0.66228e0 * t31771 + 0.258925e1 * t31812 - 0.14717333333333333333e1 * t23174 + 0.49671e0 * t31817 - 0.33114e0 * t31820 - 0.33114e0 * t31823 + 0.248355e0 * t31827 + 0.49671e0 * t31831 + 0.248355e0 * t31835 + t23178 - 0.18786444444444444445e1 * t23180;
    let t32006 = 0.40256666666666666667e0 * t23183 + 0.16504875e0 * t31841 + 0.27595e0 * t31844 + 0.11038e1 * t27153 + 0.11038e1 * t27156 - 0.14717333333333333333e1 * t27159 + 0.27595e0 * t23132 + t23172 + 0.776775e1 * t31851 - 0.16504875e0 * t31853 + 0.27595e0 * t23135 + 0.16504875e0 * t31856;
    (t31994, t32006)
}
