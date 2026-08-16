//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 711/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk711(t1415: f64, t6603: f64, t1457: f64, t6418: f64, t1265: f64, t2416: f64, t1445: f64, t447: f64, t6428: f64, t4371: f64, t884: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
