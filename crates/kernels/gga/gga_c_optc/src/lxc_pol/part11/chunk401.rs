//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 401/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk401<F: Float>(t2048: F, t88: F, t120: F, t658: F, t2040: F, t22: F) -> (F, F, F) {
    let t2050 = F::cast_from(32.0_f64) * t2048 * t88;
    let t2060 = t120 * t658;
    let t2078 = F::cast_from(1.0_f64) / t22 / t2040;
    (t2050, t2060, t2078)
}
