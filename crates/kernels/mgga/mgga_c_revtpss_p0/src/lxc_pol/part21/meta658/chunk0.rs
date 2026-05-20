//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2449/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2449<F: Float>(t3046: F, t3298: F, t4891: F, t11263: F, t3169: F, t11977: F, t3173: F, t12009: F, t12013: F, t11916: F, t11999: F, t11874: F, t16048: F) -> (F, F, F, F, F, F) {
    let t42643 = t3046 * t3298 * t4891;
    let t42656 = t3169 * t11263;
    let t42658 = t11977 * t3173;
    let t42660 = t12013 * t12009;
    let t42662 = t11999 * t11916;
    let t42675 = t11874 * t16048;
    (t42643, t42656, t42658, t42660, t42662, t42675)
}
