//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 763/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk763<F: Float>(t108: F, t34575: F, t28: F, t7212: F, t984: F, t1308: F, t6562: F, t1337: F, t6455: F, t6412: F, t7150: F, t925: F, t356: F, t461: F, t6520: F, t6454: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34576 = t34575 * t108;
    let t34577 = t28 * t34576;
    let t34580 = t7212 * t984;
    let t34581 = t28 * t34580;
    let t34584 = t1308 * t6562;
    let t34585 = t28 * t34584;
    let t34588 = t6455 * t1337;
    let t34589 = t28 * t34588;
    let t34592 = t6412 * t7150;
    let t34595 = t1308 * t925;
    let t34596 = t356 * t34595;
    let t34601 = t461 * t6520;
    let t34607 = t72 * t6454;
    (t34576, t34577, t34580, t34581, t34584, t34585, t34588, t34589, t34592, t34595, t34596, t34601, t34607)
}
