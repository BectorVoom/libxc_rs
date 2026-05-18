//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 212/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk212<F: Float>(t40: F, t699: F, t229: F, t244: F, t1: F, t243: F, t283: F, t224: F, t277: F, t36: F, t595: F) -> (F, F, F, F, F, F, F, F, F) {
    let t700 = t40 * t699;
    let t701 = F::new(2.0) * t700;
    let t702 = t229 * t244;
    let t703 = F::new(8.0) * t702;
    let t704 = t243 * t1;
    let t705 = t704 * t283;
    let t706 = F::new(0.36622894612013090108e-3) * t705;
    let t707 = t224 * t277;
    let t708 = F::new(8.0) * t707;
    let t709 = t36 * t595;
    (t700, t701, t702, t703, t704, t705, t706, t708, t709)
}
