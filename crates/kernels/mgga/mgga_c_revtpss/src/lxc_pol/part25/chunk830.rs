//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 830/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk830<F: Float>(t10430: F, t10432: F, t10435: F, t10438: F, t10442: F, t10444: F, t10469: F, t10489: F, t198: F, t765: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t2828: F, t886: F) -> (F, F) {
    let t10493 = 3.0 * t10489 * t198 * t765 + t10430 + t10432 + t10435 + t10438 + t10442 + t10444 + t10469 - t9278 + t9308 + t9316 + t9329 + t9333;
    let t10494 = t886 * t2828;
    (t10493, t10494)
}
