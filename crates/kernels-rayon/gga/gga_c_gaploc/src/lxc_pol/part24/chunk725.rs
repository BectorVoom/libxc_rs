//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 725/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk725(t590: f64, t6750: f64, t1424: f64, t1429: f64, t1436: f64, t1572: f64, t1625: f64, t2372: f64, t2385: f64, t2407: f64, t2476: f64, t4372: f64, t4540: f64, t536: f64, t6696: f64, t6700: f64, t6703: f64, t6707: f64, t6710: f64, t6712: f64, t6716: f64, t6718: f64, t6722: f64, t6724: f64, t6726: f64, t6732: f64, t6734: f64, t6737: f64, t6740: f64, t6743: f64, t6744: f64, t6747: f64) -> f64 {
    let t6751 = t6750 * t590;
    let t6754 = 0.92686455430723328401e-1_f64 * t2372 * t4372 - 0.79445533226334281486e-1_f64 * t6696 * t1424 - 0.79445533226334281486e-1_f64 * t6700 * t1424 - 0.92686455430723328401e-1_f64 * t1429 * t6703 + 0.92023022289409799224e1_f64 * t2476 * t6707 - 0.23005755572352449806e2_f64 * t6710 * t6712 + 0.13803453343411469884e2_f64 * t6716 * t6718 - 0.29822275741938360861e0_f64 * t6722 - 0.29792074959875355558e-1_f64 * t6724 + 0.47667319935800568892e0_f64 * t536 * t6726 + 0.35750489951850426669e0_f64 * t1625 * t2407 + 0.14896037479937677779e-1_f64 * t6732 - 0.21450293971110256001e1_f64 * t4540 * t6734 - 0.14300195980740170668e1_f64 * t2385 * t6737 - 0.25025342966295298669e1_f64 * t2385 * t6740 + 0.42900587942220512003e1_f64 * t6743 * t6744 + 0.95334639871601137784e0_f64 * t1572 * t6747 - 0.1022478025437886658e1_f64 * t1436 * t6751;
    t6754
}
