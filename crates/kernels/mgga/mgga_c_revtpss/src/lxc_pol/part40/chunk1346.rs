//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1346/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1346<F: Float>(t2212: F, t5789: F, t117151: F, t117153: F, t117155: F, t117161: F, t118106: F, t118108: F, t118110: F, t118154: F, t118198: F, t1456: F, t1458: F, t1914: F, t31244: F, t31512: F, t4154: F, t5790: F, t8349: F, t8433: F) -> (F,) {
    let t118203 = 2.0 * t5789 * t2212;
    let t118204 = t4154 * t8433 + t1914 * t31244 + 2.0 * t5790 * t8349 + t118106 + t118108 + t118110 + 2.0 * t117161 + 2.0 * t1456 * t31512 + t1458 * (t118154 + t118198) + t117155 + 2.0 * t117153 + t117151 + t118203;
    (t118204,)
}
