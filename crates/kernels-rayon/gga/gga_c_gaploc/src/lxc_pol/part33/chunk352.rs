//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 352/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk352(t165: f64, t475: f64, t486: f64, t1538: f64, t1266: f64, t531: f64, t1450: f64, t1453: f64, t1456: f64, t1458: f64, t1462: f64, t1509: f64, t1514: f64, t1520: f64, t1526: f64, t1530: f64, t1532: f64, t1537: f64, t190: f64, t193: f64, t199: f64, t205: f64, t525: f64, t541: f64, t557: f64, t562: f64, t581: f64, t597: f64) -> (f64, f64, f64) {
    let t1539 = t165 * t475;
    let t1540 = t486 * t1539;
    let t1541 = t1538 * t1540;
    let t1544 = t531 * t1266;
    let t1547 = -0.46011511144704899612e1_f64 * t1450 * t1453 + 0.71500979903700853338e0_f64 * t1456 * t1458 - 0.11502877786176224903e1_f64 * t1462 * t205 + 0.35750489951850426669e0_f64 * t1509 * t193 - 0.51123901271894332903e0_f64 * t199 * t1514 - 0.30674340763136599742e1_f64 * t562 * t581 - 0.79445533226334281487e-1_f64 * t190 * t1520 + 0.47667319935800568892e0_f64 * t525 * t541 + 0.11502877786176224903e2_f64 * t597 * t1526 - 0.10725146985555128001e1_f64 * t1530 * t1532 - 0.1022478025437886658e1_f64 * t1537 * t1541 - 0.35750489951850426669e0_f64 * t557 * t1544;
    (t1539, t1540, t1547)
}
