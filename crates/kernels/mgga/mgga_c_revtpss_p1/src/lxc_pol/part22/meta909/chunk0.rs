//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3111/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3111<F: Float>(t127: F, t15700: F, t15702: F, t4806: F, t16208: F, t372: F, t15666: F, t3211: F, t15656: F, t3215: F, t1025: F, t1663: F, t2434: F, t371: F) -> (F, F, F, F, F) {
    let t54667 = t15700 * t127 * t4806 * t15702;
    let t54672 = t372 * t16208;
    let t54678 = t3211 * t15666;
    let t54680 = t15656 * t3215;
    let t54687 = t1025 * t371 * t2434 * t1663;
    (t54667, t54672, t54678, t54680, t54687)
}
