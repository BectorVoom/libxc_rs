//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1506/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1506<F: Float>(t11662: F, t11710: F, t4892: F, t3046: F, t3298: F, t4891: F, t1052: F, t11243: F, t11240: F, t3144: F, t11263: F, t3169: F) -> (F, F, F, F, F) {
    let t42637 = t4892 * t11710 * t11662;
    let t42643 = t3046 * t3298 * t4891;
    let t42646 = t1052 * t11243;
    let t42648 = t11240 * t3144 * t42646;
    let t42656 = t3169 * t11263;
    (t42637, t42643, t42646, t42648, t42656)
}
