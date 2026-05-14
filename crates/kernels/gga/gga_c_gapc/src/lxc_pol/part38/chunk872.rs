//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 872/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk872<F: Float>(t11825: F, t2664: F, t3696: F, t3781: F, t761: F, t11557: F, t3780: F, t11736: F, t277: F, t612: F, t291: F, t3137: F, t959: F, t7191: F, t1026: F, t932: F) -> (F, F, F, F, F, F, F, F) {
    let t11826 = t11825 * t2664;
    let t11829 = t761 * t3696 * t3781;
    let t11831 = t11557 * t3780;
    let t11832 = t11736 * t11831;
    let t11834 = t277 * t612;
    let t11836 = t3137 * t291 * t959;
    let t11837 = t11836 * t7191;
    let t11838 = t11834 * t11837;
    let t11840 = t932 * t1026;
    (t11826, t11829, t11831, t11832, t11834, t11837, t11838, t11840)
}
