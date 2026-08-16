//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 791/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk791(t1998: f64, t2005: f64, t2009: f64, t2028: f64, t2615: f64, t2646: f64, t2649: f64, t2660: f64, t2684: f64, t5703: f64, t5724: f64, t6159: f64, t7403: f64, t7407: f64, t7411: f64, t7414: f64, t7417: f64, t7421: f64, t7424: f64, t7430: f64, t7432: f64, t7436: f64, t7439: f64, t7443: f64, t7448: f64, t7453: f64, t7459: f64, t780: f64, t825: f64) -> f64 {
    let t7462 = -0.79445533226334281486e-1_f64 * t7403 * t2028 + 0.11360866949309851756e0_f64 * t2684 * t7407 - 0.11360866949309851756e0_f64 * t825 * t7411 - 0.38342925953920749676e0_f64 * t7414 + 0.38342925953920749676e0_f64 * t7417 - 0.38342925953920749676e0_f64 * t7421 + 0.9585731488480187419e0_f64 * t7424 - 0.57514388930881124514e0_f64 * t7430 - 0.29792074959875355558e-1_f64 * t7432 - 0.14896037479937677779e-1_f64 * t7436 + 0.92023022289409799224e1_f64 * t2615 * t7439 + 0.21450293971110256002e1_f64 * t7443 * t2005 + 0.71500979903700853338e0_f64 * t5703 * t2660 - 0.71500979903700853338e0_f64 * t7448 * t2009 - 0.35750489951850426669e0_f64 * t2649 * t5724 + 0.47667319935800568892e0_f64 * t780 * t7453 - 0.46011511144704899612e1_f64 * t6159 * t2646 - 0.46011511144704899612e1_f64 * t1998 * t7459;
    t7462
}
