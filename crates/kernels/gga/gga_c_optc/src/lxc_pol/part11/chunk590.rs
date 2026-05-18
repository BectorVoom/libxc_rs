//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 590/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk590<F: Float>(t2305: F, t2310: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4806: F, t4809: F, t4812: F) -> F {
    let t4846 = t2305 + F::new(0.48461111111111111112e3) * t3640 - F::new(0.48461111111111111111e3) * t4770 + F::new(0.14538333333333333333e4) * t4774 - F::new(0.72691666666666666667e3) * t4778 + t2310 + F::new(0.10488888888888888889e3) * t3687 - F::new(0.26222222222222222222e2) * t4806 + F::new(0.15733333333333333333e3) * t4809 - F::new(0.78666666666666666667e2) * t4812;
    t4846
}
