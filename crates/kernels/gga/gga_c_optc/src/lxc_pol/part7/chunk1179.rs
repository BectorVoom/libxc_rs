//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1179/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1179<F: Float>(t2264: F, t3813: F, t123: F, t1891: F, t7492: F, t2263: F, t2672: F, t1885: F, t2274: F, t875: F, t896: F, t10925: F, t770: F) -> (F, F, F, F, F, F, F, F) {
    let t24459 = t3813 * t2264;
    let t24464 = t7492 * t1891 * t123;
    let t24468 = t2672 * t2263;
    let t24469 = t1885 * t123;
    let t24470 = t24468 * t24469;
    let t24474 = t3813 * t2274;
    let t24478 = t896 * t875;
    let t24480 = t770 * t10925;
    (t24459, t24464, t24468, t24469, t24470, t24474, t24478, t24480)
}
