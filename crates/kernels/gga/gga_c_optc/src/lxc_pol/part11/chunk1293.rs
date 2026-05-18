//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1293/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1293<F: Float>(t23860: F, t30270: F, t39413: F, t39418: F, t49240: F, t49242: F, t49393: F, t49395: F, t56969: F, t57027: F, t57037: F, t57041: F) -> F {
    let t57098 = -F::new(2.0) / F::new(3.0) * t57027 - F::new(20.0) / F::new(9.0) * t56969 - F::new(16.0) / F::new(27.0) * t39413 + F::new(16.0) / F::new(9.0) * t39418 + t23860 + F::new(8.0) / F::new(9.0) * t49240 - F::new(8.0) / F::new(3.0) * t49242 + F::new(4.0) / F::new(9.0) * t49393 + F::new(40.0) / F::new(81.0) * t49395 + F::new(112.0) / F::new(81.0) * t30270 + F::new(40.0) / F::new(9.0) * t57037 - F::new(80.0) / F::new(81.0) * t57041;
    t57098
}
