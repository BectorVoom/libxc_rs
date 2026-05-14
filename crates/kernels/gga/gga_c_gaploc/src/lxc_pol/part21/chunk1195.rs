//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1195/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1195<F: Float>(t29923: F, t31610: F, t31612: F, t31614: F, t31617: F, t31618: F, t31619: F, t31620: F, t31621: F, t31622: F) -> (F,) {
    let t38313 = t31610 - t31612 + t31614 + t29923 + t31617 - t31618 + t31619 - t31620 + t31621 - t31622;
    (t38313,)
}
