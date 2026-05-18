//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1251/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1251<F: Float>(t1057: F, t9370: F, t1052: F, t9374: F, t2754: F, t3647: F, t1524: F, t5891: F, t7536: F, t7539: F, t2640: F, t9404: F) -> (F, F, F, F, F, F, F, F) {
    let t25975 = t1057 * t9370;
    let t25977 = t1052 * t9370;
    let t25980 = t1052 * t9374;
    let t25982 = t2754 * t3647;
    let t25984 = t5891 * t1524;
    let t25986 = t7536 * t1524;
    let t25990 = t7539 * t1524;
    let t26007 = t9404 * t2640;
    (t25975, t25977, t25980, t25982, t25984, t25986, t25990, t26007)
}
