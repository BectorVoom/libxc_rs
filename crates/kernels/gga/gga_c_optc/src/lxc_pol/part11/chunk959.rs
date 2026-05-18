//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 959/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk959<F: Float>(t11700: F, t1200: F, t1565: F, t16135: F, t17574: F, t17582: F, t17585: F, t17610: F, t2886: F, t4249: F, t485: F, t5458: F, t5469: F, t9304: F) -> F {
    let t17612 = F::new(6.0) * t11700 * t5458 - t1200 * t17610 - F::new(3.0) * t16135 * t1565 + t17574 * t485 - F::new(6.0) * t9304 * t17582 + F::new(6.0) * t2886 * t17585 - F::new(3.0) * t4249 * t5469;
    t17612
}
