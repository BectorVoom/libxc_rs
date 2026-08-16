//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 676/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk676<F: Float>(t2312: F, t2322: F, t4144: F, t883: F, t485: F, t2316: F, t1624: F, t2321: F, t882: F, t2327: F, t484: F, t119: F, t3831: F, t481: F) -> (F, F, F, F, F, F) {
    let t6293 = t2312 * t2322;
    let t6295 = t883 * t4144;
    let t6296 = t485 * t6295;
    let t6297 = t2316 * t6296;
    let t6299 = t1624 * t2321;
    let t6300 = t882 * t6299;
    let t6302 = t484 * t2327;
    let t6305 = t481 * t3831 * t119;
    (t6293, t6295, t6297, t6300, t6302, t6305)
}
