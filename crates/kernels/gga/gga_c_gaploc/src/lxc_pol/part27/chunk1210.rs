//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1210/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1210<F: Float>(t209: F, t38292: F, t38295: F, t38320: F, t38337: F, t38354: F, t38355: F, t38368: F, t38369: F, t38373: F, t38382: F, t38384: F, t38385: F, t38409: F, t38444: F, t38446: F, t38451: F) -> (F,) {
    let t38456 = (t38292 + t38295 + t38320 + t38337 + t38354 + t38355 + t38368 + t38369 + t38373 + t38382 + t38384 + t38385 + t38409 + t38444 + t38446 + t38451) * t209;
    (t38456,)
}
