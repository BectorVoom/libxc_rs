//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 680/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk680<F: Float>(t1415: F, t6603: F, t1457: F, t6418: F, t1265: F, t2416: F, t1445: F, t447: F, t6428: F, t4371: F, t884: F, t898: F, t6308: F, t1450: F, t1456: F, t1562: F, t1584: F, t1596: F, t1617: F, t1646: F, t2395: F, t2399: F, t2407: F, t4771: F, t4842: F, t567: F, t6580: F, t6585: F, t6587: F, t6591: F, t6594: F, t6597: F, t6601: F) -> (F, F, F, F, F, F) {
    let t6604 = t1415 * t6603;
    let t6607 = t1457 * t6418;
    let t6610 = t2416 * t1265;
    let t6611 = t1445 * t6610;
    let t6616 = t6428 * t447;
    let t6617 = t1445 * t6616;
    let t6622 = t1445 * t6418;
    let t6625 = t4371 * t884;
    let t6626 = t898 * t6625;
    let t6628 = t1457 * t6308;
    let t6633 = t1445 * t6308;
    let t6636 = 0.38342925953920749676e0 * t6580 - 0.38342925953920749676e0 * t6585 + 0.25561950635947166452e0 * t6587 - 0.44688112439813033337e-1 * t6591 + 0.59584149919750711116e-1 * t6594 + 0.59584149919750711116e-1 * t6597 - 0.14896037479937677779e-1 * t6601 - 0.71500979903700853338e0 * t6604 * t1646 + 0.71500979903700853338e0 * t1456 * t6607 - 0.69017266717057349418e1 * t1562 * t6611 - 0.46011511144704899612e1 * t4771 * t2399 - 0.46011511144704899612e1 * t1450 * t6617 + 0.46011511144704899612e1 * t1617 * t2395 + 0.46011511144704899612e1 * t567 * t6622 + 0.89376224879626066674e-1 * t6626 - 0.71500979903700853338e0 * t4842 * t6628 + 0.35750489951850426669e0 * t1596 * t2407 - 0.46011511144704899612e1 * t1584 * t6633;
    (t6604, t6607, t6625, t6626, t6628, t6636)
}
