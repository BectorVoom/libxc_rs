//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 787/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk787<F: Float>(t3338: F, t4130: F, t2482: F, t9272: F, t12960: F, t1537: F, t3085: F, t986: F) -> (F, F, F) {
    let t41590 = t4130 * t3338;
    let t41592 = t9272 * t41590 * t2482;
    let t41594 = t1537 * t12960;
    let t41595 = 0.25561950635947166451e1 * t41594;
    let t41596 = t986 * t3085;
    (t41592, t41595, t41596)
}
