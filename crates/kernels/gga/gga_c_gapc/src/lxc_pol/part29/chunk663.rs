//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 663/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk663<F: Float>(t126: F, t667: F, t1463: F, t457: F, t1672: F, t567: F, t1180: F, t5462: F) -> (F, F, F, F) {
    let t5542 = t126 * t667;
    let t5544 = t1463 * t457;
    let t5549 = t1672 * t567;
    let t5553 = t5462 * t1180;
    (t5542, t5544, t5549, t5553)
}
