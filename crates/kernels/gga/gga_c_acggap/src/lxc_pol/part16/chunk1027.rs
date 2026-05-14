//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1027/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1027<F: Float>(t31341: F, t31377: F, t31381: F, t31382: F, t31386: F, t31390: F, t31392: F, t31407: F, t35550: F, t35553: F, t35557: F, t37610: F, t40043: F, t40045: F, t40047: F, t40050: F, t40054: F, t40057: F) -> (F,) {
    let t40059 = t31341 - t35550 + t35553 - 0.62896184579208304136e-3 * t40043 - t35557 + t37610 - t31377 - t31381 - 0.420234375e-1 * t40045 - 0.28015625e-1 * t40047 + t40050 / 8.0 + 13.0 / 96.0 * t31382 + 0.42874018118069736972e-3 * t31386 + t31390 - t31392 - t31407 - 0.34299214494455789578e-1 * t40054 + 0.64311027177104605458e-2 * t40057;
    (t40059,)
}
