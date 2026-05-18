//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1303/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1303<F: Float>(t1558: F, t231: F, t6071: F, t23327: F, t25270: F, t23297: F, t23346: F, t7045: F, t23331: F, t23293: F, t23301: F, t27261: F) -> (F, F, F, F, F, F, F) {
    let t113163 = t6071 * t1558 * t231;
    let t113171 = t25270 * t23327;
    let t113173 = t25270 * t23297;
    let t113177 = t7045 * t23346;
    let t113180 = t25270 * t23331;
    let t113182 = t25270 * t23293;
    let t113184 = t27261 * t23301;
    (t113163, t113171, t113173, t113177, t113180, t113182, t113184)
}
