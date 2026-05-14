//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 641/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk641<F: Float>(t43: F, t1205: F, t1220: F, t3084: F, t3085: F, t3125: F, t615: F, t634: F, t72: F, t88: F, t29: F, t125: F, t26: F, t1225: F, t667: F, t215: F, t2967: F) -> (F, F, F, F, F, F, F) {
    let t44 = 0.135e1 <= t43;
    let t3129 = piecewise3(t44, t3084, -8.0 / 3.0 * t1205 * t634 - 8.0 / 3.0 * t615 * t1220 - 8.0 / 3.0 * t3085 * t88 - 8.0 / 3.0 * t72 * t3125);
    let t3130 = t29 * t3129;
    let t3131 = t3130 * t125;
    let t3132 = t26 * t3131;
    let t3135 = t1225 * t667;
    let t3136 = t26 * t3135;
    let t3141 = t2967 * t215;
    (t3129, t3130, t3131, t3132, t3135, t3136, t3141)
}
