//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 745/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk745<F: Float>(t2717: F, t773: F, t2653: F, t783: F, t701: F, t7258: F, t1445: F, t1998: F, t2005: F, t2009: F, t2028: F, t2615: F, t2646: F, t2649: F, t2660: F, t2684: F, t5703: F, t5724: F, t6159: F, t7403: F, t7407: F, t7411: F, t7414: F, t7417: F, t7421: F, t7424: F, t7430: F, t7432: F, t7436: F, t7439: F, t7443: F, t780: F, t825: F) -> (F,) {
    let t7448 = t773 * t2717;
    let t7453 = t2653 * t783;
    let t7458 = t7258 * t701;
    let t7459 = t1445 * t7458;
    let t7462 = -0.79445533226334281486e-1 * t7403 * t2028 + 0.11360866949309851756e0 * t2684 * t7407 - 0.11360866949309851756e0 * t825 * t7411 - 0.38342925953920749676e0 * t7414 + 0.38342925953920749676e0 * t7417 - 0.38342925953920749676e0 * t7421 + 0.9585731488480187419e0 * t7424 - 0.57514388930881124514e0 * t7430 - 0.29792074959875355558e-1 * t7432 - 0.14896037479937677779e-1 * t7436 + 0.92023022289409799224e1 * t2615 * t7439 + 0.21450293971110256002e1 * t7443 * t2005 + 0.71500979903700853338e0 * t5703 * t2660 - 0.71500979903700853338e0 * t7448 * t2009 - 0.35750489951850426669e0 * t2649 * t5724 + 0.47667319935800568892e0 * t780 * t7453 - 0.46011511144704899612e1 * t6159 * t2646 - 0.46011511144704899612e1 * t1998 * t7459;
    (t7462,)
}
