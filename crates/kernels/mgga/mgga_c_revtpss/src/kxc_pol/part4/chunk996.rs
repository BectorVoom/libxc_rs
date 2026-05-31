//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 996/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk996<F: Float>(t116: F, t2319: F, t2389: F, t705: F, t2258: F, t750: F, t706: F, t157: F, t36: F, t2401: F, t200: F, t45: F) -> (F, F, F, F, F, F) {
    let t10416 = t2319 * t116;
    let t10428 = t705 * t2389;
    let t10436 = t750 * t2258;
    let t10437 = t706 * t10436;
    let t10439 = t36 * t157;
    let t10443 = t2401 * t750;
    let t10446 = F::cast_from(1.0_f64) / t200 / t45;
    (t10416, t10428, t10437, t10439, t10443, t10446)
}
