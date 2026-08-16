//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 666/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk666<F: Float>(t1200: F, t1565: F, t2886: F, t4249: F, t485: F, t5454: F, t5458: F, t5469: F, t275: F, t176: F, sigma2: F) -> (F, F) {
    let t5471 = -t1200 * t5469 - F::cast_from(2.0_f64) * t4249 * t1565 + F::cast_from(2.0_f64) * t2886 * t5458 + t5454 * t485;
    let t5472 = t5471 * t275;
    let t5474 = t176 * t5472 * sigma2;
    (t5471, t5474)
}
