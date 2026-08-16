//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3448/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448<F: Float>(t19380: F, t999: F, t3075: F, t6258: F, t4946: F, t15654: F, t1678: F, t19748: F, t4866: F, t20089: F, t3153: F, t11249: F, t6271: F) -> (F, F, F, F, F, F, F) {
    let t64831 = t19380 * t999;
    let t64835 = t6258 * t3075;
    let t64841 = t4946 * t999;
    let t64845 = t15654 * t1678;
    let t64848 = t19748 * t4866;
    let t64854 = t20089 * t3153;
    let t64861 = t6271 * t11249;
    (t64831, t64835, t64841, t64845, t64848, t64854, t64861)
}
