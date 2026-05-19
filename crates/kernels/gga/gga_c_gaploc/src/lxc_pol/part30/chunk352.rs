//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 352/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk352<F: Float>(t165: F, t475: F, t486: F, t1538: F, t1266: F, t531: F, t1450: F, t1453: F, t1456: F, t1458: F, t1462: F, t1509: F, t1514: F, t1520: F, t1526: F, t1530: F, t1532: F, t1537: F, t190: F, t193: F, t199: F, t205: F, t525: F, t541: F, t557: F, t562: F, t581: F, t597: F) -> (F, F, F) {
    let t1539 = t165 * t475;
    let t1540 = t486 * t1539;
    let t1541 = t1538 * t1540;
    let t1544 = t531 * t1266;
    let t1547 = -F::cast_from(0.46011511144704899612e1_f64) * t1450 * t1453 + F::cast_from(0.71500979903700853338e0_f64) * t1456 * t1458 - F::cast_from(0.11502877786176224903e1_f64) * t1462 * t205 + F::cast_from(0.35750489951850426669e0_f64) * t1509 * t193 - F::cast_from(0.51123901271894332903e0_f64) * t199 * t1514 - F::cast_from(0.30674340763136599742e1_f64) * t562 * t581 - F::cast_from(0.79445533226334281487e-1_f64) * t190 * t1520 + F::cast_from(0.47667319935800568892e0_f64) * t525 * t541 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t1526 - F::cast_from(0.10725146985555128001e1_f64) * t1530 * t1532 - F::cast_from(0.1022478025437886658e1_f64) * t1537 * t1541 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t1544;
    (t1539, t1540, t1547)
}
