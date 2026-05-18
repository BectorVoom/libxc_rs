//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1061/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1061<F: Float>(t1038: F, t18813: F, t2579: F, t2801: F, t18822: F, t3787: F, t15515: F, t7592: F, t1: F, t932: F, t28415: F, t8140: F) -> (F, F, F, F, F) {
    let t28427 = t2579 * t2801 * t1038 * t18813;
    let t28472 = t3787 * t1038 * t18822;
    let t28517 = t7592 * t15515;
    let t28524 = t932 * t1;
    let t28526 = t28415 * t8140;
    (t28427, t28472, t28517, t28524, t28526)
}
