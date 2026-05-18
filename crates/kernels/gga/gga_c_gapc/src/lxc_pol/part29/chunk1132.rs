//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1132/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1132<F: Float>(t1882: F, t277: F, t9959: F, t11954: F, t2981: F, t876: F, t1: F, t1736: F, t2206: F, t311: F, t3383: F, t8675: F) -> (F, F, F, F) {
    let t33998 = t277 * t1882 * t9959;
    let t34001 = t11954 * t2981 * t876;
    let t34005 = t311 * t2206 * t1736 * t1;
    let t34007 = t34005 * t8675 * t3383;
    (t33998, t34001, t34005, t34007)
}
