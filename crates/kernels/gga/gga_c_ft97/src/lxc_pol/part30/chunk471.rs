//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 471/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk471<F: Float>(t1403: F, t1427: F, t247: F, t7437: F, t7443: F, t7487: F, t7491: F, t7542: F, t7547: F, t7554: F, t7558: F, t7560: F) -> F {
    let t7565 = t7437 * t1427 / F::new(6.0) - t1403 * t7443 / F::new(3.0) + t1403 * t7487 / F::new(6.0) + t1403 * t7491 / F::new(3.0) - t247 * t7558 + F::new(2.0) * t7560 - F::new(4.0) * t7542 + F::new(4.0) * t7547 - F::new(2.0) * t7554;
    t7565
}
