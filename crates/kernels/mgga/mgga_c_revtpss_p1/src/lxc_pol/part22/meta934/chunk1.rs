//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3166/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166<F: Float>(t1012: F, t44958: F, t13026: F, t140: F, t1222: F, t16715: F, t1224: F, t5052: F, t697: F, t12915: F, t17344: F, t17345: F, t247: F) -> (F, F, F, F, F) {
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    let t57486 = t1222 * t57484 * t16715;
    let t57490 = t1222 * t697 * t1224 * t5052;
    let t57508 = t17344 * t247 * t12915 * t17345;
    (t57480, t57484, t57486, t57490, t57508)
}
