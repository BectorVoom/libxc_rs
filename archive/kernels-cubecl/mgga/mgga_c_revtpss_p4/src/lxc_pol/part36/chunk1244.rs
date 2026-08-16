//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1244/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1244<F: Float>(t25898: F, t7925: F, t94849: F, t25953: F, t27884: F, t10073: F, t25938: F, t27836: F, t7289: F, t97925: F, t2470: F, t27872: F) -> (F, F, F, F, F) {
    let t97956 = t94849 * t25898 * t7925;
    let t97985 = t27884 * t25953;
    let t98003 = t10073 * t27836 * t25938;
    let t98011 = t7289 * t97925;
    let t98028 = t27872 * t2470;
    (t97956, t97985, t98003, t98011, t98028)
}
