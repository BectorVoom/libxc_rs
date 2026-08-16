//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1392/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1392(t12000: f64, t1564: f64, t188: f64, t38362: f64, t12064: f64, t1531: f64, t11981: f64, t12068: f64, t12070: f64, t12134: f64, t12138: f64, t1265: f64, t1323: f64, t1445: f64, t1450: f64, t1456: f64, t1457: f64, t1530: f64, t1562: f64, t30770: f64, t30773: f64, t30778: f64, t30779: f64, t34541: f64, t34548: f64, t38388: f64, t38414: f64, t4614: f64, t4679: f64, t4730: f64, t475: f64, t4953: f64, t6744: f64) -> f64 {
    let t38613 = t1564 * t12000;
    let t38622 = t188 * t38362;
    let t38640 = t12064 * t1531;
    let t38643 = 0.38342925953920749677e0_f64 * t30770 + 0.53964118009221795842e0_f64 * t30773 - t30778 + 0.76685851907841499354e0_f64 * t30779 - 0.13803453343411469884e2_f64 * t4953 * t12070 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t38613 * t475 - 0.69017266717057349418e1_f64 * t1562 * t1445 * t12068 * t1265 + 0.42900587942220512003e1_f64 * t38622 * t6744 + 0.71500979903700853338e0_f64 * t4679 * t12138 + 0.71500979903700853338e0_f64 * t1456 * t1457 * t38414 + 0.35750489951850426669e0_f64 * t1456 * t1457 * t38388 + 0.46011511144704899612e1_f64 * t4730 * t1445 * t11981 * t1323 - 0.61348681526273199482e1_f64 * t1450 * t4614 * t12134 - 0.25025342966295298669e1_f64 * t1530 * t38640 - t34541 - t34548;
    t38643
}
