//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2680/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2680<F: Float>(t11710: F, t19872: F, t3091: F, t19968: F, t3111: F, t15850: F, t4817: F, t11921: F, t19399: F, t247: F, t4837: F, t15752: F, t19741: F) -> (F, F, F, F, F) {
    let t66731 = t3091 * t11710 * t19872;
    let t66739 = t19968 * t3111;
    let t66747 = t15850 * t4817;
    let t66752 = t4837 * t247 * t11921 * t19399;
    let t66758 = t19741 * t15752;
    (t66731, t66739, t66747, t66752, t66758)
}
