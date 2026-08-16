//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1195/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1195<F: Float>(t15853: F, t17874: F, t311: F, t4043: F, t519: F, t7113: F, t7547: F, t7549: F, t1882: F, t277: F, t9959: F, t11954: F, t2981: F, t876: F) -> (F, F, F, F) {
    let t33988 = t311 * t15853 * t4043 * t519 * t17874;
    let t33991 = t7547 * t7113 * t7549;
    let t33998 = t277 * t1882 * t9959;
    let t34001 = t11954 * t2981 * t876;
    (t33988, t33991, t33998, t34001)
}
