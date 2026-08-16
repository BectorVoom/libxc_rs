//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3147/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147<F: Float>(t3362: F, t462: F, t51959: F, t52011: F, t44348: F, t44919: F, t12327: F, t3391: F, t5079: F, t12331: F, t1134: F, t16926: F, t3390: F) -> (F, F, F, F, F, F) {
    let t58027 = t462 * t3362;
    let t58029 = t52011 * t58027 * t51959;
    let t58032 = t52011 * t44348 * t51959;
    let t58035 = t52011 * t44919 * t51959;
    let t58038 = t12327 * t5079 * t3391;
    let t58041 = t12331 * t5079 * t3391;
    let t58044 = t3390 * t16926 * t1134;
    (t58029, t58032, t58035, t58038, t58041, t58044)
}
