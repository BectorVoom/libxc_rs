//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1976/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1976<F: Float>(t4181: F, t603: F, t4187: F, t38: F, t7714: F, t2247: F, t1493: F, t644: F, t77: F, t13272: F, t6957: F, t4173: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t28116 = t603 * t4181;
    let t28119 = t603 * t4187;
    let t28126 = t38 * t7714;
    let t28127 = t2247 * t28126;
    let t28133 = t77 * t1493 * t644;
    let t28138 = t13272 * t6957;
    let t28141 = t4173 * t607;
    (t28116, t28119, t28126, t28127, t28133, t28138, t28141)
}
