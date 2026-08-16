//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3701/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3701<F: Float>(t3568: F, t6573: F, t12915: F, t17344: F, t20747: F, t247: F, t1261: F, t44693: F, t6421: F, t12910: F, t12916: F, t20857: F) -> (F, F, F, F) {
    let t70120 = t6573 * t3568;
    let t70129 = t17344 * t247 * t12915 * t20747;
    let t70133 = t1261 * t247 * t44693 * t6421;
    let t70140 = t12910 * t12916 * t20857;
    (t70120, t70129, t70133, t70140)
}
