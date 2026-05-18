//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1336/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1336<F: Float>(t23769: F, t23770: F, t30189: F, t30270: F, t49378: F, t49381: F, t56978: F, t56981: F, t56984: F, t56988: F, t56991: F, t56994: F) -> F {
    let t58143 = -F::new(26169.0) * t56978 + F::new(0.58153333333333333332e4) * t56981 - F::new(0.19384444444444444444e4) * t56984 - F::new(2832.0) * t56988 + F::new(0.62933333333333333332e3) * t56991 + F::new(0.94399999999999999998e3) * t56994 + F::new(0.93234567901234567903e3) * t30189 + t23769 + t23770 + F::new(0.932345679012345679e2) * t49378 + F::new(0.20977777777777777778e3) * t49381 + F::new(0.30153580246913580247e4) * t30270;
    t58143
}
