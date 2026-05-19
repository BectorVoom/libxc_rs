//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 812/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk812<F: Float>(t1645: F, t1716: F, t1710: F, t2667: F, t1445: F, t2628: F, t2657: F, t1457: F, t7259: F, t1966: F, t2004: F, t2061: F, t2070: F, t2087: F, t2103: F, t2625: F, t2631: F, t2638: F, t2639: F, t2654: F, t5577: F, t5771: F, t7697: F, t7700: F, t7703: F, t7712: F, t7716: F, t7720: F, t7723: F, t7727: F, t7730: F, t7733: F, t780: F, t813: F) -> (F, F, F) {
    let t7736 = t1645 * t1716;
    let t7739 = t2667 * t1710;
    let t7740 = t1445 * t7739;
    let t7743 = t2657 * t2628;
    let t7747 = t1457 * t7259;
    let t7750 = -F::cast_from(0.1022478025437886658e1_f64) * t5577 * t2625 - F::cast_from(0.1022478025437886658e1_f64) * t1966 * t7697 + F::cast_from(0.71500979903700853338e0_f64) * t2004 * t7700 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t7703 + F::cast_from(0.71500979903700853338e0_f64) * t2070 * t2654 + F::cast_from(0.35750489951850426669e0_f64) * t2061 * t2654 + F::cast_from(0.71500979903700853338e0_f64) * t780 * t7712 + F::cast_from(0.14896037479937677779e-1_f64) * t7716 + F::cast_from(0.29792074959875355558e-1_f64) * t7720 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t7723 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t7727 - F::cast_from(0.21450293971110256002e1_f64) * t7730 * t2639 - F::cast_from(0.21450293971110256002e1_f64) * t7733 * t2639 - F::cast_from(0.10725146985555128001e1_f64) * t2638 * t7736 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t7740 - F::cast_from(0.59584149919750711116e-1_f64) * t7743 + F::cast_from(0.14300195980740170668e1_f64) * t5771 * t2631 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t7747;
    (t7736, t7743, t7750)
}
