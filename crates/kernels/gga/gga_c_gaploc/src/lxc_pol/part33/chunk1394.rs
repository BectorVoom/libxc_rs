//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1394/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1394<F: Float>(t12064: F, t540: F, t3689: F, t4529: F, t11986: F, t11987: F, t12044: F, t12057: F, t12060: F, t12109: F, t12113: F, t1328: F, t1391: F, t1392: F, t1445: F, t1456: F, t1457: F, t1562: F, t1599: F, t1628: F, t30897: F, t30900: F, t30901: F, t38414: F, t38429: F, t38436: F, t4527: F, t4614: F, t4673: F, t4762: F, t4950: F, t536: F, t541: F, t557: F, t567: F, t574: F, t587: F) -> F {
    let t38688 = t12064 * t540;
    let t38694 = t4529 * t3689;
    let t38719 = -F::new(0.11360866949309851756e0) * t587 * t1391 * t1392 * t11986 - F::new(0.61348681526273199482e1) * t574 * t1628 * t12109 + F::new(0.47667319935800568892e0) * t12113 * t541 + F::new(0.47667319935800568892e0) * t536 * t38688 - F::new(0.62115540045351614476e2) * t1562 * t1445 * t38429 + F::new(0.27606906686822939767e2) * t4527 * t1445 * t38694 * t1328 + F::new(0.61348681526273199482e1) * t567 * t4614 * t11987 - F::new(0.35750489951850426669e0) * t12060 * t4762 - F::new(0.21450293971110256002e1) * t557 * t1457 * t38436 + F::new(0.46011511144704899612e1) * t567 * t1445 * t38414 - t30897 - t30900 + F::new(0.38342925953920749677e0) * t30901 + F::new(0.47667319935800568892e0) * t1456 * t4673 * t11987 - F::new(0.21450293971110256002e1) * t1599 * t1457 * t12044 + F::new(0.14300195980740170668e1) * t4950 * t12057;
    t38719
}
