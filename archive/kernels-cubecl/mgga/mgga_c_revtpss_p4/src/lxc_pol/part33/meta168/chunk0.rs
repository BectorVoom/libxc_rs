//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 830/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk830<F: Float>(t225: F, t4469: F, t1568: F, t213: F, t1580: F, t779: F, t689: F, t1579: F, t72: F, t686: F) -> (F, F, F, F, F, F) {
    let t4470 = t4469 * t225;
    let t4474 = t213 * t1568;
    let t4477 = t779 * t1580;
    let t4478 = t689 * t4477;
    let t4480 = t1579 * t72;
    let t4481 = t4480 * t686;
    (t4470, t4474, t4477, t4478, t4480, t4481)
}
