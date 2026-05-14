//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1217/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1217<F: Float>(t12045: F, t1641: F, t34394: F, t34397: F, t34404: F, t34406: F, t34410: F, t34414: F, t34415: F, t34416: F, t34418: F, t34420: F, t34423: F, t34425: F, t34431: F, t34435: F) -> (F,) {
    let t38541 = -0.92023022289409799224e1 * t1641 * t12045 - t34394 - t34397 - t34404 - t34406 - t34410 - t34414 - t34415 + t34416 + t34418 - t34420 + t34423 + t34425 + t34431 + t34435;
    (t38541,)
}
