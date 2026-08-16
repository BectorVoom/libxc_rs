//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1394/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1394(t12064: f64, t540: f64, t3689: f64, t4529: f64, t11986: f64, t11987: f64, t12044: f64, t12057: f64, t12060: f64, t12109: f64, t12113: f64, t1328: f64, t1391: f64, t1392: f64, t1445: f64, t1456: f64, t1457: f64, t1562: f64, t1599: f64, t1628: f64, t30897: f64, t30900: f64, t30901: f64, t38414: f64, t38429: f64, t38436: f64, t4527: f64, t4614: f64, t4673: f64, t4762: f64, t4950: f64, t536: f64, t541: f64, t557: f64, t567: f64, t574: f64, t587: f64) -> f64 {
    let t38688 = t12064 * t540;
    let t38694 = t4529 * t3689;
    let t38719 = -0.11360866949309851756e0_f64 * t587 * t1391 * t1392 * t11986 - 0.61348681526273199482e1_f64 * t574 * t1628 * t12109 + 0.47667319935800568892e0_f64 * t12113 * t541 + 0.47667319935800568892e0_f64 * t536 * t38688 - 0.62115540045351614476e2_f64 * t1562 * t1445 * t38429 + 0.27606906686822939767e2_f64 * t4527 * t1445 * t38694 * t1328 + 0.61348681526273199482e1_f64 * t567 * t4614 * t11987 - 0.35750489951850426669e0_f64 * t12060 * t4762 - 0.21450293971110256002e1_f64 * t557 * t1457 * t38436 + 0.46011511144704899612e1_f64 * t567 * t1445 * t38414 - t30897 - t30900 + 0.38342925953920749677e0_f64 * t30901 + 0.47667319935800568892e0_f64 * t1456 * t4673 * t11987 - 0.21450293971110256002e1_f64 * t1599 * t1457 * t12044 + 0.14300195980740170668e1_f64 * t4950 * t12057;
    t38719
}
