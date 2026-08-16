//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 713/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk713<F: Float>(t1457: F, t6308: F, t1445: F, t1450: F, t1456: F, t1562: F, t1584: F, t1596: F, t1617: F, t1646: F, t2395: F, t2399: F, t2407: F, t4771: F, t4842: F, t567: F, t6580: F, t6585: F, t6587: F, t6591: F, t6594: F, t6597: F, t6601: F, t6604: F, t6607: F, t6611: F, t6617: F, t6622: F, t6626: F) -> (F, F) {
    let t6628 = t1457 * t6308;
    let t6633 = t1445 * t6308;
    let t6636 = F::cast_from(0.38342925953920749676e0_f64) * t6580 - F::cast_from(0.38342925953920749676e0_f64) * t6585 + F::cast_from(0.25561950635947166452e0_f64) * t6587 - F::cast_from(0.44688112439813033337e-1_f64) * t6591 + F::cast_from(0.59584149919750711116e-1_f64) * t6594 + F::cast_from(0.59584149919750711116e-1_f64) * t6597 - F::cast_from(0.14896037479937677779e-1_f64) * t6601 - F::cast_from(0.71500979903700853338e0_f64) * t6604 * t1646 + F::cast_from(0.71500979903700853338e0_f64) * t1456 * t6607 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t6611 - F::cast_from(0.46011511144704899612e1_f64) * t4771 * t2399 - F::cast_from(0.46011511144704899612e1_f64) * t1450 * t6617 + F::cast_from(0.46011511144704899612e1_f64) * t1617 * t2395 + F::cast_from(0.46011511144704899612e1_f64) * t567 * t6622 + F::cast_from(0.89376224879626066674e-1_f64) * t6626 - F::cast_from(0.71500979903700853338e0_f64) * t4842 * t6628 + F::cast_from(0.35750489951850426669e0_f64) * t1596 * t2407 - F::cast_from(0.46011511144704899612e1_f64) * t1584 * t6633;
    (t6628, t6636)
}
