//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1176/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1176<F: Float>(t1852: F, t8653: F, t8666: F, t3197: F, t6134: F, t3191: F, t8662: F, t39: F, t6525: F, t6528: F, t1232: F, t674: F, t2062: F, t6536: F, t2065: F, t8678: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25842 = t1852 * t8653;
    let t25844 = t1852 * t8666;
    let t25860 = t6134 * t3197;
    let t25862 = t6134 * t3191;
    let t25864 = t1852 * t8662;
    let t25900 = t6525 * t39 * t6528;
    let t25901 = t1232 * t674;
    let t25907 = t2062 * t39 * t6536;
    let t25911 = t8678 * t2065;
    (t25842, t25844, t25860, t25862, t25864, t25900, t25901, t25907, t25911)
}
