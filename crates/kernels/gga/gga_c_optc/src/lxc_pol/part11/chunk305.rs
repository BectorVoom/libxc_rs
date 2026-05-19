//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 305/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk305<F: Float>(t1261: F, t1274: F, t1278: F, t135: F, t626: F, t628: F, t636: F, t656: F) -> F {
    let t1281 = -t626 - t628 * t1261 / F::new(48.0) - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t1274 - t656 - F::cast_from(0.10866451862235947318e-1_f64) * t135 * t1278;
    t1281
}
