//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1751/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1751<F: Float>(t2389: F, t705: F, t2258: F, t750: F, t706: F, t157: F, t36: F, t2401: F, t200: F, t45: F, t202: F, t57: F) -> (F, F, F, F, F, F, F) {
    let t10428 = t705 * t2389;
    let t10436 = t750 * t2258;
    let t10437 = t706 * t10436;
    let t10439 = t36 * t157;
    let t10443 = t2401 * t750;
    let t10446 = F::new(1.0) / t200 / t45;
    let t10457 = F::new(1.0) / t202 / t57;
    (t10428, t10436, t10437, t10439, t10443, t10446, t10457)
}
