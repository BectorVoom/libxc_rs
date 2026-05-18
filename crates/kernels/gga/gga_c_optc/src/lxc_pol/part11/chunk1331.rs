//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1331/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1331<F: Float>(t13632: F, t16922: F, t23537: F, t2722: F, t277: F, t39204: F, t4038: F, t4044: F, t49197: F, t49223: F, t56676: F, t56681: F, t56686: F, t56693: F, t57628: F, t57995: F, t95: F, t962: F) -> F {
    let t58004 = F::new(2.0) / F::new(3.0) * t49197 - t56676 + t56681 + F::new(2.0) / F::new(3.0) * t49223 - t23537 - F::new(8.0) / F::new(27.0) * t39204 + t56686 + F::new(8.0) / F::new(3.0) * t13632 * t16922 - t56693 + F::new(0.25844881434903430496e-2) * t95 * t277 * t57995 * t962 + F::new(6.0) * t4038 * t2722 * t4044 * t57628;
    t58004
}
