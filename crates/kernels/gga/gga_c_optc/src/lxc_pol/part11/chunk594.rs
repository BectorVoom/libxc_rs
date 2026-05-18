//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 594/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk594<F: Float>(t4884: F, t818: F, t2520: F, t4868: F, t2444: F, t3640: F, t4770: F, t4774: F, t4778: F, t232: F, t4818: F, t799: F) -> (F, F, F, F, F) {
    let t4885 = t4884 * t818;
    let t4888 = t4868 * t2520;
    let t4895 = t2444 + F::new(0.11872222222222222222e-1) * t3640 - F::new(0.11872222222222222222e-1) * t4770 + F::new(0.35616666666666666666e-1) * t4774 - F::new(0.17808333333333333333e-1) * t4778;
    let t4897 = F::new(0.62182e-1) * t4895 * t232;
    let t4898 = t4818 * t799;
    (t4885, t4888, t4895, t4897, t4898)
}
