//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 809/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk809<F: Float>(t38339: F, t38375: F, t1820: F, t8360: F, t108: F, t1537: F, t1538: F, t1761: F, t1920: F, t348: F, t37302: F, t37349: F, t37419: F, t38294: F, t38300: F, t38304: F, t437: F, t438: F, t497: F, t7734: F, t8198: F, t8199: F, t8588: F) -> (F, F) {
    let t38376 = t38339 + t38375;
    let t38379 = t8360 * t1820;
    let t38381 = -3.0 * t1537 * t8198 * t108 - 4.0 * t438 * t8588 - 6.0 * t1761 * t1920 - 4.0 * t7734 * t497 - t348 * (t37302 + t37349 + t37419 + t38294) * t108 - 8.0 * t38300 - 4.0 * t8199 * t497 - 8.0 * t38304 - 6.0 * t1538 * t1920 - t38376 * t437 * t108 - 12.0 * t38379;
    (t38379, t38381)
}
