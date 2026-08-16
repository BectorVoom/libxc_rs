//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 809/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk809(t1645: f64, t1716: f64, t1710: f64, t2667: f64, t1445: f64, t2628: f64, t2657: f64, t1457: f64, t7259: f64, t1966: f64, t2004: f64, t2061: f64, t2070: f64, t2087: f64, t2103: f64, t2625: f64, t2631: f64, t2638: f64, t2639: f64, t2654: f64, t5577: f64, t5771: f64, t7697: f64, t7700: f64, t7703: f64, t7712: f64, t7716: f64, t7720: f64, t7723: f64, t7727: f64, t7730: f64, t7733: f64, t780: f64, t813: f64) -> (f64, f64, f64) {
    let t7736 = t1645 * t1716;
    let t7739 = t2667 * t1710;
    let t7740 = t1445 * t7739;
    let t7743 = t2657 * t2628;
    let t7747 = t1457 * t7259;
    let t7750 = -0.1022478025437886658e1_f64 * t5577 * t2625 - 0.1022478025437886658e1_f64 * t1966 * t7697 + 0.71500979903700853338e0_f64 * t2004 * t7700 + 0.35750489951850426669e0_f64 * t2004 * t7703 + 0.71500979903700853338e0_f64 * t2070 * t2654 + 0.35750489951850426669e0_f64 * t2061 * t2654 + 0.71500979903700853338e0_f64 * t780 * t7712 + 0.14896037479937677779e-1_f64 * t7716 + 0.29792074959875355558e-1_f64 * t7720 - 0.92023022289409799224e1_f64 * t813 * t7723 - 0.46011511144704899612e1_f64 * t813 * t7727 - 0.21450293971110256002e1_f64 * t7730 * t2639 - 0.21450293971110256002e1_f64 * t7733 * t2639 - 0.10725146985555128001e1_f64 * t2638 * t7736 - 0.69017266717057349418e1_f64 * t2087 * t7740 - 0.59584149919750711116e-1_f64 * t7743 + 0.14300195980740170668e1_f64 * t5771 * t2631 + 0.14300195980740170668e1_f64 * t2103 * t7747;
    (t7736, t7743, t7750)
}
