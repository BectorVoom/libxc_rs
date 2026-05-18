//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1297/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1297<F: Float>(t23926: F, t23927: F, t30189: F, t30270: F, t49378: F, t49381: F, t49385: F, t49387: F, t49393: F, t56988: F, t56991: F, t56994: F, t56997: F, t56999: F) -> F {
    let t57148 = -F::new(0.298026e1) * t56988 + F::new(0.66228e0) * t56991 + F::new(0.99342e0) * t56994 + F::new(0.98115555555555555556e0) * t30189 + t23926 + t23927 - F::new(0.247573125e0) * t56997 + F::new(0.3300975e0) * t56999 + F::new(0.98115555555555555555e-1) * t49378 + F::new(0.22076e0) * t49381 + F::new(0.12524296296296296297e1) * t30270 - F::new(0.16102666666666666667e1) * t49385 + F::new(0.24154e1) * t49387 + F::new(0.40256666666666666668e0) * t49393;
    t57148
}
