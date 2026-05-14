//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1063/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1063<F: Float>(t4192: F, t6914: F, t4157: F, t786: F, t810: F, t10963: F, t10978: F, t10985: F, t10987: F, t10990: F, t10996: F, t11003: F, t11005: F, t6762: F, t6884: F, t8908: F, t9028: F) -> (F, F, F, F) {
    let t11040 = 0.16081979498692535067e2 * t6914 * t4192;
    let t11041 = t4157 * t786;
    let t11043 = 1.0 * t11041 * t810;
    let t11056 = 0.264729375e1 * t10985 - 0.3529725e1 * t10987 - 0.17648625e1 * t10990 + 0.3529725e1 * t10996 - t6884 + 0.68863333333333333333e0 * t6762 + 0.13772666666666666667e1 * t8908 - t9028 - 0.516475e0 * t10963 + 0.1549425e1 * t10978 - 0.157790625e0 * t11003 + 0.6311625e0 * t11005;
    (t11040, t11041, t11043, t11056)
}
