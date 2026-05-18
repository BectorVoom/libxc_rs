//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 721/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk721<F: Float>(t590: F, t6750: F, t1424: F, t1429: F, t1436: F, t1572: F, t1625: F, t2372: F, t2385: F, t2407: F, t2476: F, t4372: F, t4540: F, t536: F, t6696: F, t6700: F, t6703: F, t6707: F, t6710: F, t6712: F, t6716: F, t6718: F, t6722: F, t6724: F, t6726: F, t6732: F, t6734: F, t6737: F, t6740: F, t6743: F, t6744: F, t6747: F) -> F {
    let t6751 = t6750 * t590;
    let t6754 = F::new(0.92686455430723328401e-1) * t2372 * t4372 - F::new(0.79445533226334281486e-1) * t6696 * t1424 - F::new(0.79445533226334281486e-1) * t6700 * t1424 - F::new(0.92686455430723328401e-1) * t1429 * t6703 + F::new(0.92023022289409799224e1) * t2476 * t6707 - F::new(0.23005755572352449806e2) * t6710 * t6712 + F::new(0.13803453343411469884e2) * t6716 * t6718 - F::new(0.29822275741938360861e0) * t6722 - F::new(0.29792074959875355558e-1) * t6724 + F::new(0.47667319935800568892e0) * t536 * t6726 + F::new(0.35750489951850426669e0) * t1625 * t2407 + F::new(0.14896037479937677779e-1) * t6732 - F::new(0.21450293971110256001e1) * t4540 * t6734 - F::new(0.14300195980740170668e1) * t2385 * t6737 - F::new(0.25025342966295298669e1) * t2385 * t6740 + F::new(0.42900587942220512003e1) * t6743 * t6744 + F::new(0.95334639871601137784e0) * t1572 * t6747 - F::new(0.1022478025437886658e1) * t1436 * t6751;
    t6754
}
