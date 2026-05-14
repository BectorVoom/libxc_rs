//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 459/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk459<F: Float>(t3153: F, t3603: F, t1244: F, t3598: F, t3594: F, t471: F, t1121: F, t414: F, t66: F, t474: F, t479: F, t3089: F) -> (F, F, F, F, F, F, F, F) {
    let t3604 = t3153 * t3603;
    let t3609 = t1244 * t3598;
    let t3610 = t3594 * t3609;
    let t3611 = t3153 * t471;
    let t3617 = 1.0 / t414 / t1121;
    let t3618 = t66 * t3617;
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    (t3604, t3609, t3610, t3611, t3617, t3618, t3623, t3624)
}
