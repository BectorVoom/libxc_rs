//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 464/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk464<F: Float>(t641: F, t928: F, t655: F, t2299: F, t332: F, t330: F, t197: F, t617: F, t968: F, t2188: F, t918: F, t1904: F, t2660: F, t327: F, t328: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2741 = t928 * t641;
    let t2744 = t928 * t655;
    let t2747 = t332 * t2299;
    let t2748 = t330 * t2747;
    let t2749 = t197 * t2748;
    let t2752 = t617 * t968;
    let t2755 = t332 * t2188;
    let t2756 = t918 * t2755;
    let t2757 = t197 * t2756;
    let t2760 = t2660 * t1904;
    let t2761 = M_PI * t327;
    let t2762 = t328 * t328;
    let t2763 = 1.0 / t2762;
    (t2741, t2744, t2749, t2752, t2757, t2760, t2761, t2762, t2763)
}
