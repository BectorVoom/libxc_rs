//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 784/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk784<F: Float>(t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t157: F, t36: F, t200: F, t45: F, t202: F, t57: F, t2441: F, t9303: F, t10115: F, t258: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10368 = 1.0 / t59 / t58;
    let t10379 = 1232.0 / 27.0 * t10199;
    let t10389 = 1.0 / t78 / t2851;
    let t10398 = 1.0 / t81 / t3361;
    let t10439 = t36 * t157;
    let t10446 = 1.0 / t200 / t45;
    let t10457 = 1.0 / t202 / t57;
    let t10501 = 0.26019841438354088051e-2 * t9303 * t2441;
    let t10503 = 0.11044544084478153697e-3 * t10115 * t258;
    (t10368, t10379, t10389, t10398, t10439, t10446, t10457, t10501, t10503)
}
