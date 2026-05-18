//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 808/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk808<F: Float>(t1645: F, t1716: F, t1710: F, t2667: F, t1445: F, t2628: F, t2657: F, t1457: F, t7259: F, t1966: F, t2004: F, t2061: F, t2070: F, t2087: F, t2103: F, t2625: F, t2631: F, t2638: F, t2639: F, t2654: F, t5577: F, t5771: F, t7697: F, t7700: F, t7703: F, t7712: F, t7716: F, t7720: F, t7723: F, t7727: F, t7730: F, t7733: F, t780: F, t813: F) -> (F, F, F) {
    let t7736 = t1645 * t1716;
    let t7739 = t2667 * t1710;
    let t7740 = t1445 * t7739;
    let t7743 = t2657 * t2628;
    let t7747 = t1457 * t7259;
    let t7750 = -F::new(0.1022478025437886658e1) * t5577 * t2625 - F::new(0.1022478025437886658e1) * t1966 * t7697 + F::new(0.71500979903700853338e0) * t2004 * t7700 + F::new(0.35750489951850426669e0) * t2004 * t7703 + F::new(0.71500979903700853338e0) * t2070 * t2654 + F::new(0.35750489951850426669e0) * t2061 * t2654 + F::new(0.71500979903700853338e0) * t780 * t7712 + F::new(0.14896037479937677779e-1) * t7716 + F::new(0.29792074959875355558e-1) * t7720 - F::new(0.92023022289409799224e1) * t813 * t7723 - F::new(0.46011511144704899612e1) * t813 * t7727 - F::new(0.21450293971110256002e1) * t7730 * t2639 - F::new(0.21450293971110256002e1) * t7733 * t2639 - F::new(0.10725146985555128001e1) * t2638 * t7736 - F::new(0.69017266717057349418e1) * t2087 * t7740 - F::new(0.59584149919750711116e-1) * t7743 + F::new(0.14300195980740170668e1) * t5771 * t2631 + F::new(0.14300195980740170668e1) * t2103 * t7747;
    (t7736, t7743, t7750)
}
