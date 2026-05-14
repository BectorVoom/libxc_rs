//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 653/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk653<F: Float>(t1328: F, t2416: F, t6320: F, t1529: F, t888: F, t1217: F, t885: F, t1222: F, t1210: F, t78: F, t119: F, t481: F) -> (F, F, F, F, F, F) {
    let t6321 = t2416 * t1328;
    let t6322 = t6320 * t6321;
    let t6325 = t1529 * t888;
    let t6328 = t1217 * t885;
    let t6334 = t1222 * t885;
    let t6336 = t78 * t1210;
    let t6338 = t481 * t6336 * t119;
    (t6321, t6322, t6325, t6328, t6334, t6338)
}
