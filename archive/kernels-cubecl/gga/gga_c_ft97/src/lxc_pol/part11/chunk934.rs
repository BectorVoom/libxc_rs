//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 934/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk934<F: Float>(t2304: F, t5: F, t9470: F, t2253: F, t8626: F, t8650: F, t8662: F, t8636: F, t179: F, t37406: F, t3628: F, t634: F) -> (F, F, F, F, F, F, F, F) {
    let t39390 = t2304 * t2304;
    let t39396 = t5 * t9470;
    let t39402 = t2253 * t8626;
    let t39404 = t2253 * t8650;
    let t39413 = t2253 * t8662;
    let t39415 = t2253 * t8636;
    let t39417 = t179 * t37406;
    let t39422 = t3628 * t634;
    (t39390, t39396, t39402, t39404, t39413, t39415, t39417, t39422)
}
