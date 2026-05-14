//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1170/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1170<F: Float>(t1052: F, t9370: F, t9374: F, t2754: F, t3647: F, t1524: F, t5891: F, t7536: F, t7539: F, t2640: F, t9404: F, t7544: F, t3639: F, t7520: F, t2676: F, t1112: F, t483: F, t9369: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25977 = t1052 * t9370;
    let t25980 = t1052 * t9374;
    let t25982 = t2754 * t3647;
    let t25984 = t5891 * t1524;
    let t25986 = t7536 * t1524;
    let t25990 = t7539 * t1524;
    let t26007 = t9404 * t2640;
    let t26010 = t7544 * t1524;
    let t26012 = t3639 * t7520;
    let t26020 = t9404 * t2676;
    let t26023 = t9369 * t483 * t1112;
    (t25977, t25980, t25982, t25984, t25986, t25990, t26007, t26010, t26012, t26020, t26023)
}
