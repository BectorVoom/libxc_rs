//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1146/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1146<F: Float>(t100174: F, t100180: F, t100185: F, t100188: F, t100192: F, t100198: F, t100203: F, t100208: F, t92140: F, t92143: F, t92161: F, t1767: F, t1871: F, t25893: F, t5675: F, t942: F) -> (F, F) {
    let t100210 = t100174 / 27.0 + 2.0 / 27.0 * t92140 + 8.0 / 27.0 * t92143 + 4.0 / 3.0 * t100180 - t100185 - 4.0 / 9.0 * t100188 - 2.0 / 9.0 * t100192 - t92161 / 27.0 + 2.0 / 3.0 * t100198 + t100203 / 12.0 + t100208 / 12.0;
    let t100214 = t25893 * t1871 * t5675 * t942 * t1767;
    (t100210, t100214)
}
