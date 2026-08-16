//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1815/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1815<F: Float>(t26338: F, t81228: F, t81326: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F, t1834: F, t794: F, t6891: F, t22704: F, t26355: F) -> (F, F, F, F, F, F) {
    let t90524 = t81228 * t81326 * t26338;
    let t90533 = t22892 * t80645 * t7691;
    let t90541 = t6883 * t26206;
    let t90544 = t794 * t1834;
    let t90546 = t22892 * t90544 * t6891;
    let t90549 = t22704 * t81326 * t26355;
    (t90524, t90533, t90541, t90544, t90546, t90549)
}
