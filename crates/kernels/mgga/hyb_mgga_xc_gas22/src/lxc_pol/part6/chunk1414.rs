//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1414/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1414<F: Float>(t1123: F, t1129: F, t3957: F, t15681: F, t5471: F, t1304: F, t26729: F, t3740: F, t3748: F, t11478: F, t26730: F, t1535: F, t1605: F, sigma0: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t30570 = t3957 * t1123 * t1129;
    let t30571 = t15681 * t30570;
    let t30574 = t5471 * tau0;
    let t30576 = t26729 * t1304;
    let t30577 = t3740 * sigma0;
    let t30578 = t30576 * t30577;
    let t30585 = t3748 * sigma0;
    let t30586 = t30576 * t30585;
    let t30595 = t11478 * sigma0;
    let t30596 = t26730 * t30595;
    let t30599 = t1605 * t1535;
    (t30570, t30571, t30574, t30578, t30586, t30596, t30599)
}
