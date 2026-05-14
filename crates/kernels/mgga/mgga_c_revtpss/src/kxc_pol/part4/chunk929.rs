//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 929/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk929<F: Float>(t10308: F, t29: F, t46: F, t47: F, t58: F, t59: F, t10199: F, t2851: F, t78: F, t3361: F, t81: F, t116: F, t2319: F, t2389: F, t705: F, t2258: F, t750: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10309 = t29 * t10308;
    let t10355 = 1.0 / t47 / t46;
    let t10368 = 1.0 / t59 / t58;
    let t10379 = 1232.0 / 27.0 * t10199;
    let t10389 = 1.0 / t78 / t2851;
    let t10398 = 1.0 / t81 / t3361;
    let t10416 = t2319 * t116;
    let t10428 = t705 * t2389;
    let t10436 = t750 * t2258;
    (t10309, t10355, t10368, t10379, t10389, t10398, t10416, t10428, t10436)
}
