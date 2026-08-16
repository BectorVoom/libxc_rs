//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 764/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk764<F: Float>(t3994: F, t808: F, t9845: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9365: F, t9374: F, t9376: F, t9389: F, t9391: F) -> (F, F) {
    let t9846 = t808 * t3994;
    let t9847 = t9845 * t9846;
    let t9849 = -t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 + t9365 - t9374 - t9376 - t9389 - t9391;
    (t9847, t9849)
}
