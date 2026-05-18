//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 739/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk739<F: Float>(t2920: F, t8492: F, t2925: F, t1469: F, t2891: F, t1488: F, t517: F, t8356: F, t3954: F, t475: F, t115: F, t8379: F) -> (F, F, F, F, F, F) {
    let t8493 = t2920 * t8492;
    let t8494 = t8493 * t2925;
    let t8496 = t1469 * t2891;
    let t8498 = t1488 * t2891;
    let t8500 = t8356 * t517;
    let t8501 = t475 * t3954;
    let t8502 = t8500 * t8501;
    let t8504 = t8379 * t115;
    (t8493, t8494, t8496, t8498, t8502, t8504)
}
