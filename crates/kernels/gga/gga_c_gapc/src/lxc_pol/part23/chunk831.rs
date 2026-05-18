//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 831/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk831<F: Float>(t3343: F, t9689: F, t1026: F, t2787: F, t937: F, t641: F, t7073: F, t2579: F, t2701: F, t8895: F, t277: F, t5463: F) -> (F, F, F, F, F) {
    let t9690 = t3343 * t9689;
    let t9692 = t2787 * t1026;
    let t9693 = t9692 * t937;
    let t9695 = t7073 * t641;
    let t9697 = t8895 * t2579 * t2701;
    let t9698 = t9695 * t9697;
    let t9700 = t277 * t5463;
    (t9690, t9693, t9695, t9698, t9700)
}
