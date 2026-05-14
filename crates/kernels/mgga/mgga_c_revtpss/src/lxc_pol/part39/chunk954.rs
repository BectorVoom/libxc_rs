//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 954/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk954<F: Float>(t136: F, t1412: F, t220: F, t124: F, t1398: F, t3938: F, t9816: F, t4003: F, t4056: F, t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F, t14: F, t588: F) -> (F, F, F, F, F, F, F) {
    let t9817 = t1412 * t136;
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9840 = t4003 * t4056;
    let t9845 = t2735 * t4086;
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9854 = 24.0 * t9342 * t521;
    let t9855 = t14 * t588;
    (t9818, t9822, t9840, t9845, t9847, t9854, t9855)
}
