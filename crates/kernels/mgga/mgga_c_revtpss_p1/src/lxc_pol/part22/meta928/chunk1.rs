//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3154/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3154<F: Float>(t17544: F, t3708: F, t12915: F, t16771: F, t247: F, t5384: F, t17763: F, t3636: F, t13085: F, t5391: F, t12881: F, t5381: F) -> (F, F, F, F, F) {
    let t57063 = t3708 * t17544;
    let t57070 = t5384 * t247 * t12915 * t16771;
    let t57075 = t17763 * t3636;
    let t57077 = t5391 * t13085;
    let t57094 = t5381 * t12881;
    (t57063, t57070, t57075, t57077, t57094)
}
