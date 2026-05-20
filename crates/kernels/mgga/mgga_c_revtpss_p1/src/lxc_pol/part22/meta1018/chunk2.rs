//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3523/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3523<F: Float>(t1011: F, t6288: F, t697: F, t11710: F, t19872: F, t3091: F, t19979: F, t3153: F, t372: F, t19968: F, t3111: F, t15850: F, t4817: F) -> (F, F, F, F, F) {
    let t66721 = t1011 * t697 * t6288;
    let t66731 = t3091 * t11710 * t19872;
    let t66734 = t372 * t19979 * t3153;
    let t66739 = t19968 * t3111;
    let t66747 = t15850 * t4817;
    (t66721, t66731, t66734, t66739, t66747)
}
