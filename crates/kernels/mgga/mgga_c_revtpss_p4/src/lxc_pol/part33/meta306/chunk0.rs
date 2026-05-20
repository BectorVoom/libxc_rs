//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1302/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1302<F: Float>(t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t157: F, t36: F, t200: F, t45: F) -> (F, F, F, F, F, F, F) {
    let t10355 = F::new(1.0) / t47 / t46;
    let t10368 = F::new(1.0) / t59 / t58;
    let t10379 = F::new(1232.0) / F::new(27.0) * t10199;
    let t10389 = F::new(1.0) / t78 / t2851;
    let t10398 = F::new(1.0) / t81 / t3361;
    let t10439 = t36 * t157;
    let t10446 = F::new(1.0) / t200 / t45;
    (t10355, t10368, t10379, t10389, t10398, t10439, t10446)
}
