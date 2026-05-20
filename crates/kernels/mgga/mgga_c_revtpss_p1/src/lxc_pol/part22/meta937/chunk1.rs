//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3172/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3172<F: Float>(t1222: F, t16733: F, t17240: F, t12772: F, t17678: F, t5340: F, t17683: F, t5331: F, t12832: F, t17620: F, t17412: F, t3636: F) -> (F, F, F, F, F) {
    let t57749 = t1222 * t17240 * t16733;
    let t57770 = t5340 * t12772 * t17678;
    let t57773 = t5331 * t12772 * t17683;
    let t57780 = t12832 * t17620;
    let t57786 = t17412 * t3636;
    (t57749, t57770, t57773, t57780, t57786)
}
