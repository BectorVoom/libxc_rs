//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 791/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk791<F: Float>(t1998: F, t2005: F, t2009: F, t2028: F, t2615: F, t2646: F, t2649: F, t2660: F, t2684: F, t5703: F, t5724: F, t6159: F, t7403: F, t7407: F, t7411: F, t7414: F, t7417: F, t7421: F, t7424: F, t7430: F, t7432: F, t7436: F, t7439: F, t7443: F, t7448: F, t7453: F, t7459: F, t780: F, t825: F) -> F {
    let t7462 = -F::cast_from(0.79445533226334281486e-1_f64) * t7403 * t2028 + F::cast_from(0.11360866949309851756e0_f64) * t2684 * t7407 - F::cast_from(0.11360866949309851756e0_f64) * t825 * t7411 - F::cast_from(0.38342925953920749676e0_f64) * t7414 + F::cast_from(0.38342925953920749676e0_f64) * t7417 - F::cast_from(0.38342925953920749676e0_f64) * t7421 + F::cast_from(0.9585731488480187419e0_f64) * t7424 - F::cast_from(0.57514388930881124514e0_f64) * t7430 - F::cast_from(0.29792074959875355558e-1_f64) * t7432 - F::cast_from(0.14896037479937677779e-1_f64) * t7436 + F::cast_from(0.92023022289409799224e1_f64) * t2615 * t7439 + F::cast_from(0.21450293971110256002e1_f64) * t7443 * t2005 + F::cast_from(0.71500979903700853338e0_f64) * t5703 * t2660 - F::cast_from(0.71500979903700853338e0_f64) * t7448 * t2009 - F::cast_from(0.35750489951850426669e0_f64) * t2649 * t5724 + F::cast_from(0.47667319935800568892e0_f64) * t780 * t7453 - F::cast_from(0.46011511144704899612e1_f64) * t6159 * t2646 - F::cast_from(0.46011511144704899612e1_f64) * t1998 * t7459;
    t7462
}
