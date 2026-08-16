//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2026/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026<F: Float>(t27551: F, t7327: F, t135: F, t24847: F, t7284: F, t1090: F, t24821: F, t1089: F, t1235: F, t11708: F, t24728: F, t11713: F, t11715: F, t11717: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86102 = t24821 * t1090;
    let t86116 = t7327 * t1235 * t1089;
    let t86140 = t11708 * t24728;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    (t86077, t86094, t86102, t86116, t86140, t86146)
}
