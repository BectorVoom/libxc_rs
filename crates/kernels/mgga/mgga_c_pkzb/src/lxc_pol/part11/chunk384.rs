//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 384/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk384<F: Float>(t1571: F, t1589: F, t465: F, t471: F, t204: F, t492: F) -> (F, F, F) {
    let t1590 = t1571 * t1589;
    let t1593 = t465 * t471;
    let t1596 = 0.35616666666666666666e-1 * t204 * t1593 * t492;
    (t1590, t1593, t1596)
}
