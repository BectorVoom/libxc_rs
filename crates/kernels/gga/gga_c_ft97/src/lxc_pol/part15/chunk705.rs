//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 705/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk705<F: Float>(t1787: F, t20145: F, t20103: F, t8291: F, t20107: F, t11669: F, t11720: F, t16373: F, t16404: F, t16406: F, t16442: F, t16444: F, t16446: F, t462: F, t8301: F) -> (F, F, F, F) {
    let t20372 = t1787 * t20145;
    let t20381 = t8291 * t20103;
    let t20384 = t1787 * t20107;
    let t20387 = t462 * t20372 - F::new(4.0) / F::new(9.0) * t11720 + t16404 - F::new(2.0) * t16406 - F::new(4.0) / F::new(3.0) * t11669 - F::new(2.0) / F::new(3.0) * t16373 - t8301 - F::new(2.0) / F::new(3.0) * t16442 + t16444 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t16446 - F::new(2.0) * t462 * t20381 - F::new(2.0) * t462 * t20384;
    (t20372, t20381, t20384, t20387)
}
