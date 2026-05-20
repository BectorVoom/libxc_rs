//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1988/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1988<F: Float>(t7048: F, t822: F, t25300: F, t9285: F, t25299: F, t7059: F, t9288: F, t7064: F, t25305: F, t136: F, t2457: F, t7082: F) -> (F, F, F, F, F, F) {
    let t92864 = t822 * t7048;
    let t92868 = t25300 * t9285;
    let t92870 = F::cast_from(0.68540937416128198417e-2_f64) * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = F::cast_from(0.39982213492741449076e-1_f64) * t7064 * t92871;
    let t92875 = F::cast_from(0.91399340044406952588e-2_f64) * t25305 * t92868;
    let t92894 = t7082 * t136 * t2457;
    (t92864, t92870, t92871, t92873, t92875, t92894)
}
