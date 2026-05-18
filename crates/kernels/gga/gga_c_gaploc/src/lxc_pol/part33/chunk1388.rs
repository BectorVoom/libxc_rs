//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1388/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1388<F: Float>(t12000: F, t1564: F, t188: F, t38362: F, t12064: F, t1531: F, t11981: F, t12068: F, t12070: F, t12134: F, t12138: F, t1265: F, t1323: F, t1445: F, t1450: F, t1456: F, t1457: F, t1530: F, t1562: F, t30770: F, t30773: F, t30778: F, t30779: F, t34541: F, t34548: F, t38388: F, t38414: F, t4614: F, t4679: F, t4730: F, t475: F, t4953: F, t6744: F) -> F {
    let t38613 = t1564 * t12000;
    let t38622 = t188 * t38362;
    let t38640 = t12064 * t1531;
    let t38643 = F::new(0.38342925953920749677e0) * t30770 + F::new(0.53964118009221795842e0) * t30773 - t30778 + F::new(0.76685851907841499354e0) * t30779 - F::new(0.13803453343411469884e2) * t4953 * t12070 - F::new(0.13803453343411469884e2) * t1562 * t1445 * t38613 * t475 - F::new(0.69017266717057349418e1) * t1562 * t1445 * t12068 * t1265 + F::new(0.42900587942220512003e1) * t38622 * t6744 + F::new(0.71500979903700853338e0) * t4679 * t12138 + F::new(0.71500979903700853338e0) * t1456 * t1457 * t38414 + F::new(0.35750489951850426669e0) * t1456 * t1457 * t38388 + F::new(0.46011511144704899612e1) * t4730 * t1445 * t11981 * t1323 - F::new(0.61348681526273199482e1) * t1450 * t4614 * t12134 - F::new(0.25025342966295298669e1) * t1530 * t38640 - t34541 - t34548;
    t38643
}
