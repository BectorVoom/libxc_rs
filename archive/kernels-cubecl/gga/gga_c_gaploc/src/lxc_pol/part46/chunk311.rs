//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 311/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk311<F: Float>(t2513: F, t2515: F, t2520: F, t2522: F, t471: F, t64: F, t931: F) -> (F, F) {
    let t2524 = -F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t2513 + F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t2515 - F::cast_from(7.0_f64) / F::cast_from(8192.0_f64) * t2520 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t2522;
    let t2530 = t2524 * t471 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t931 * t64 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t2513 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t2522;
    (t2524, t2530)
}
