//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 757/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk757<F: Float>(t1866: F, t37269: F, t446: F, t1588: F, t1647: F, t7824: F, t1882: F, t7830: F, t379: F, t8183: F, t1564: F, t1651: F, t1755: F, t432: F, t7966: F, t10: F, t11175: F, t83: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t37271 = t446 * t1866 * t37269;
    let t37273 = t1647 * t1588;
    let t37275 = t446 * t7824 * t37273;
    let t37277 = t1882 * t7830;
    let t37279 = t379 * t8183;
    let t37281 = t446 * t1564 * t37279;
    let t37283 = t1651 * t1755;
    let t37285 = t446 * t1564 * t37283;
    let t37287 = t7966 * t432;
    let t37289 = t446 * t1564 * t37287;
    let t37292 = t10 * t11175 * t83;
    (t37271, t37273, t37275, t37277, t37279, t37281, t37283, t37285, t37287, t37289, t37292)
}
