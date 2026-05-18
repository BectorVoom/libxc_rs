//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 470/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk470<F: Float>(t2294: F, t2296: F, t2301: F, t2302: F, t2315: F, t350: F, t974: F, t979: F, t275: F, t176: F, sigma0: F) -> (F, F) {
    let t2317 = t2294 * t350 - F::new(2.0) * t2296 * t979 + F::new(2.0) * t2301 * t2302 - t974 * t2315;
    let t2318 = t2317 * t275;
    let t2320 = t176 * t2318 * sigma0;
    (t2317, t2320)
}
