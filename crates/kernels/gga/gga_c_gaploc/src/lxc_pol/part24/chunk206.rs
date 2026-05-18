//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 206/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk206<F: Float>(t304: F, t747: F, t178: F, t670: F, t108: F, t260: F, t14: F, t1: F, t271: F, t509: F, t110: F, t257: F) -> (F, F, F, F, F, F, F) {
    let t748 = t304 * t747;
    let t749 = t670 * t178;
    let t752 = t260 * t108;
    let t753 = t752 * t14;
    let t754 = t271 * t1;
    let t755 = t754 * t509;
    let t758 = t110 * t257;
    (t748, t749, t752, t753, t754, t755, t758)
}
