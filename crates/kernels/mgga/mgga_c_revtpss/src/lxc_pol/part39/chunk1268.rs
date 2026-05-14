//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1268/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1268<F: Float>(t1042: F, t17236: F, t1224: F, t140: F, t5052: F, t1222: F, t16738: F, t5308: F, t16742: F, t16733: F, t16771: F, t247: F, t3719: F, t3636: F, t5391: F, t5381: F) -> (F, F, F, F, F, F, F, F) {
    let t17237 = t1042 * t17236;
    let t17240 = t140 * t1224;
    let t17241 = t17240 * t5052;
    let t17243 = t1222 * t17241 / 216.0;
    let t17244 = t5308 * t16738;
    let t17247 = t5308 * t16742;
    let t17250 = t5308 * t16733;
    let t17254 = t247 * t3719 * t16771;
    let t17258 = 0.10162730220579493208e-2 * t5391 * t3636;
    let t17260 = 0.19055119163586549765e-3 * t5381 * t3636;
    (t17237, t17243, t17244, t17247, t17250, t17254, t17258, t17260)
}
