//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1344/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1344<F: Float>(t114401: F, t508: F, t651: F, t29583: F, t7898: F, t1450: F, t22809: F, t2014: F, t7237: F, t1907: F, t6922: F, t28196: F, t28197: F) -> (F, F, F, F) {
    let t114773 = F::cast_from(2.0_f64) * t651 * t508 * t114401;
    let t114775 = F::cast_from(18.0_f64) * t7898 * t29583;
    let t114776 = t1450 * t22809;
    let t114779 = F::cast_from(3.0_f64) * t2014 * t7237 * t114776;
    let t114780 = t1907 * t6922;
    let t114783 = F::cast_from(6.0_f64) * t28196 * t28197 * t114780;
    (t114773, t114775, t114779, t114783)
}
