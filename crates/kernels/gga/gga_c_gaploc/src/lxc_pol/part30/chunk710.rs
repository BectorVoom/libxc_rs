//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 710/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk710<F: Float>(t1415: F, t6603: F, t1457: F, t6418: F, t1265: F, t2416: F, t1445: F, t447: F, t6428: F, t4371: F, t884: F, t898: F) -> (F, F, F, F, F, F, F) {
    let t6604 = t1415 * t6603;
    let t6607 = t1457 * t6418;
    let t6610 = t2416 * t1265;
    let t6611 = t1445 * t6610;
    let t6616 = t6428 * t447;
    let t6617 = t1445 * t6616;
    let t6622 = t1445 * t6418;
    let t6625 = t4371 * t884;
    let t6626 = t898 * t6625;
    (t6604, t6607, t6611, t6617, t6622, t6625, t6626)
}
