//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 343/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk343<F: Float>(t1461: F, t198: F, t1189: F, t178: F, t108: F, t400: F, t14: F, t435: F, t75: F, t341: F, t112: F, t19: F) -> (F, F, F, F, F) {
    let t1462 = t1461 * t198;
    let t1465 = t1189 * t178;
    let t1468 = t400 * t108;
    let t1469 = t1468 * t14;
    let t1474 = t75 * t435;
    let t1475 = t1474 * t341;
    let t1476 = t112 * t19;
    (t1462, t1465, t1469, t1475, t1476)
}
