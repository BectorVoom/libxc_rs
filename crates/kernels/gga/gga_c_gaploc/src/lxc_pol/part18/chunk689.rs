//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 689/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk689<F: Float>(t1344: F, t1645: F, t188: F, t6316: F, t1340: F, t2345: F, t4673: F, t493: F, t6519: F, t590: F, t1424: F, t1429: F, t1436: F, t1572: F, t1625: F, t2372: F, t2385: F, t2407: F, t2476: F, t4372: F, t4540: F, t536: F, t6696: F, t6700: F, t6703: F, t6707: F, t6710: F, t6712: F, t6716: F, t6718: F, t6722: F, t6724: F, t6726: F, t6732: F, t6734: F, t6737: F) -> (F, F, F, F) {
    let t6740 = t1645 * t1344;
    let t6743 = t188 * t6316;
    let t6744 = t1645 * t1340;
    let t6747 = t4673 * t2345;
    let t6750 = t493 * t6519;
    let t6751 = t6750 * t590;
    let t6754 = 0.92686455430723328401e-1 * t2372 * t4372 - 0.79445533226334281486e-1 * t6696 * t1424 - 0.79445533226334281486e-1 * t6700 * t1424 - 0.92686455430723328401e-1 * t1429 * t6703 + 0.92023022289409799224e1 * t2476 * t6707 - 0.23005755572352449806e2 * t6710 * t6712 + 0.13803453343411469884e2 * t6716 * t6718 - 0.29822275741938360861e0 * t6722 - 0.29792074959875355558e-1 * t6724 + 0.47667319935800568892e0 * t536 * t6726 + 0.35750489951850426669e0 * t1625 * t2407 + 0.14896037479937677779e-1 * t6732 - 0.21450293971110256001e1 * t4540 * t6734 - 0.14300195980740170668e1 * t2385 * t6737 - 0.25025342966295298669e1 * t2385 * t6740 + 0.42900587942220512003e1 * t6743 * t6744 + 0.95334639871601137784e0 * t1572 * t6747 - 0.1022478025437886658e1 * t1436 * t6751;
    (t6740, t6744, t6750, t6754)
}
