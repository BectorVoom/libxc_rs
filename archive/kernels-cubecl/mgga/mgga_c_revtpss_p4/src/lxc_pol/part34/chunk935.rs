//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 935/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk935<F: Float>(t22188: F, t22191: F, t22196: F, t22762: F, t22763: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F) {
    let t22764 = F::cast_from(12.0_f64) * t22188;
    let t22765 = F::cast_from(12.0_f64) * t22191;
    let t22766 = F::cast_from(3.0_f64) * t22196;
    let t22767 = t22762 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391 - t22763 - t22764 - t22765 + t22766;
    (t22764, t22765, t22766, t22767)
}
