//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2117/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2117<F: Float>(t10957: F, t3053: F, t271: F, t2770: F, t10321: F, t1041: F, t248: F, t3051: F, t10459: F, t3117: F, t10469: F, t990: F) -> (F, F, F, F, F) {
    let t42303 = t10957 * t3053;
    let t42308 = F::cast_from(1.0_f64) / t271 / t2770;
    let t42322 = t1041 * t248 * t3051 * t10321;
    let t42324 = t3117 * t10459;
    let t42332 = t990 * t10469;
    (t42303, t42308, t42322, t42324, t42332)
}
