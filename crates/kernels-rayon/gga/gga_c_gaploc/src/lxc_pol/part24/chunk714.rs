//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 714/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk714(t1457: f64, t6308: f64, t1445: f64, t1450: f64, t1456: f64, t1562: f64, t1584: f64, t1596: f64, t1617: f64, t1646: f64, t2395: f64, t2399: f64, t2407: f64, t4771: f64, t4842: f64, t567: f64, t6580: f64, t6585: f64, t6587: f64, t6591: f64, t6594: f64, t6597: f64, t6601: f64, t6604: f64, t6607: f64, t6611: f64, t6617: f64, t6622: f64, t6626: f64) -> (f64, f64) {
    let t6628 = t1457 * t6308;
    let t6633 = t1445 * t6308;
    let t6636 = 0.38342925953920749676e0_f64 * t6580 - 0.38342925953920749676e0_f64 * t6585 + 0.25561950635947166452e0_f64 * t6587 - 0.44688112439813033337e-1_f64 * t6591 + 0.59584149919750711116e-1_f64 * t6594 + 0.59584149919750711116e-1_f64 * t6597 - 0.14896037479937677779e-1_f64 * t6601 - 0.71500979903700853338e0_f64 * t6604 * t1646 + 0.71500979903700853338e0_f64 * t1456 * t6607 - 0.69017266717057349418e1_f64 * t1562 * t6611 - 0.46011511144704899612e1_f64 * t4771 * t2399 - 0.46011511144704899612e1_f64 * t1450 * t6617 + 0.46011511144704899612e1_f64 * t1617 * t2395 + 0.46011511144704899612e1_f64 * t567 * t6622 + 0.89376224879626066674e-1_f64 * t6626 - 0.71500979903700853338e0_f64 * t4842 * t6628 + 0.35750489951850426669e0_f64 * t1596 * t2407 - 0.46011511144704899612e1_f64 * t1584 * t6633;
    (t6628, t6636)
}
