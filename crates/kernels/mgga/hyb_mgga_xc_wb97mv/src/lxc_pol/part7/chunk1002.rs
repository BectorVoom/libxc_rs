//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1002/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1002<F: Float>(t1522: F, t7917: F, t1519: F, t2848: F, t3723: F, t646: F, t1148: F, t1111: F, t3813: F, t1117: F, t2856: F, t1153: F, t2860: F, t1114: F, t3791: F, t2869: F, t515: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9947 = t7917 * t1522;
    let t9954 = t2848 * t1519;
    let t9974 = t3723 * t646;
    let t9977 = t1148 * t2848;
    let t9978 = t3813 * t1111;
    let t9981 = t1117 * t2856;
    let t9984 = t2860 * t1153;
    let t9985 = t3791 * t1114;
    let t9988 = t515 * t2869;
    (t9947, t9954, t9974, t9977, t9978, t9981, t9984, t9985, t9988)
}
