//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1033/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1033<F: Float>(t140: F, t3698: F, t1012: F, t13026: F, t1234: F, t5390: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t12268: F, t3617: F) -> (F, F, F, F, F, F, F, F) {
    let t17471 = t140 * t3698;
    let t17475 = t1012 * t13026;
    let t17505 = t1234 * t5390;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17550 = t3617 * t12268;
    (t17471, t17475, t17505, t17524, t17525, t17528, t17529, t17550)
}
