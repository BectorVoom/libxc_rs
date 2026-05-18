//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 458/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk458<F: Float>(t2334: F, t2338: F, t2341: F, t2352: F, t2357: F, t2364: F, t2368: F, t2376: F, t2462: F, t2478: F, t2516: F) -> F {
    let t2518 = F::new(4.0) / F::new(9.0) * t2334;
    let t2526 = -t2478 / F::new(4.0) + t2516 / F::new(2.0) + t2518 + F::new(2.0) / F::new(9.0) * t2338 + F::new(2.0) / F::new(3.0) * t2341 - F::new(2.0) / F::new(9.0) * t2352 + F::new(2.0) / F::new(3.0) * t2357 + F::new(2.0) / F::new(3.0) * t2364 - t2368 / F::new(3.0) + F::new(2.0) * t2376 - t2462;
    t2526
}
