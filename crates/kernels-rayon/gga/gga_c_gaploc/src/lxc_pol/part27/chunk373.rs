//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 373/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk373(t1604: f64, t1605: f64, t1609: f64, t1613: f64, t1617: f64, t1621: f64, t1625: f64, t1629: f64, t1632: f64, t1635: f64, t1638: f64, t1641: f64, t1644: f64, t1646: f64, t193: f64, t557: f64, t567: f64, t571: f64, t574: f64, t576: f64, t597: f64) -> f64 {
    let t1649 = 0.71500979903700853338e0_f64 * t1604 * t1605 + 0.23005755572352449806e1_f64 * t597 * t1609 - 0.23005755572352449806e1_f64 * t574 * t1613 + 0.23005755572352449806e1_f64 * t1617 * t571 + 0.11502877786176224903e1_f64 * t567 * t1621 + 0.35750489951850426669e0_f64 * t1625 * t193 + 0.61348681526273199483e1_f64 * t597 * t1629 + 0.30674340763136599742e1_f64 * t567 * t1632 - 0.61348681526273199483e1_f64 * t574 * t1635 - 0.47667319935800568892e0_f64 * t557 * t1638 - 0.46011511144704899612e1_f64 * t1641 * t576 - 0.71500979903700853338e0_f64 * t1644 * t1646;
    t1649
}
