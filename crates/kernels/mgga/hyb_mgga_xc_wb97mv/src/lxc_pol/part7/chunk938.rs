//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 938/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk938<F: Float>(t1234: F, t2029: F, t1312: F, t2163: F, t26: F, t1314: F, t136: F, t2015: F, t2966: F, t3003: F, t3305: F, t676: F, t683: F, t8632: F, t8848: F, t8852: F, t8855: F, t8860: F, t8862: F, t8864: F, t8866: F) -> (F, F, F, F) {
    let t8870 = t1234 * t2029 / 32.0;
    let t8871 = t2163 * t1312;
    let t8872 = t26 * t8871;
    let t8879 = -t8632 - 3.0 / 64.0 * t136 * t8848 + t8852 / 288.0 + t683 * t3003 * t8855 / 32.0 - t8860 + t8862 / 96.0 + t8864 / 96.0 + 3.0 / 16.0 * t2966 * t8866 - t8870 - 3.0 / 64.0 * t136 * t8872 - 3.0 / 64.0 * t2015 * t1314 - 3.0 / 32.0 * t676 * t3305;
    (t8870, t8871, t8872, t8879)
}
