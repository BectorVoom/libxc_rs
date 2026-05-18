//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1126/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1126<F: Float>(t6892: F, t8950: F, t1727: F, t8891: F, t3448: F, t5384: F, t16399: F, t8996: F, t6966: F, t8968: F, t17043: F, t9000: F) -> (F, F, F, F, F, F) {
    let t24219 = t6892 * t8950;
    let t24251 = t1727 * t8891;
    let t24259 = t5384 * t3448;
    let t24269 = t16399 * t8996;
    let t24272 = t6966 * t8968;
    let t24282 = t17043 * t9000;
    (t24219, t24251, t24259, t24269, t24272, t24282)
}
