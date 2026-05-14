//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 487/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk487<F: Float>(t1385: F, t1419: F, t198: F, t531: F, t1448: F, t1450: F, t565: F) -> (F, F, F, F, F) {
    let t4118 = t1385 * t1419;
    let t4139 = t198 * t531;
    let t4140 = t1448 * t1450;
    let t4146 = t565 * t565;
    let t4147 = 1.0 / t4146;
    (t4118, t4139, t4140, t4146, t4147)
}
