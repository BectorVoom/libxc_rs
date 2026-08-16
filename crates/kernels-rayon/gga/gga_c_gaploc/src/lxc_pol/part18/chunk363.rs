//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 363/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk363(t1549: f64, t1552: f64, t1556: f64, t1562: f64, t1567: f64, t1572: f64, t1573: f64, t1577: f64, t1580: f64, t1584: f64, t1586: f64, t1590: f64, t1596: f64, t1599: f64, t193: f64, t530: f64, t532: f64, t536: f64, t541: f64, t558: f64, t574: f64, t602: f64) -> f64 {
    let t1602 = 0.71500979903700853338e0_f64 * t1549 * t193 - 0.35750489951850426669e0_f64 * t530 * t1552 - 0.71500979903700853338e0_f64 * t1556 * t532 - 0.69017266717057349418e1_f64 * t1562 * t1567 + 0.71500979903700853338e0_f64 * t1572 * t1573 - 0.46011511144704899612e1_f64 * t574 * t1577 + 0.46011511144704899612e1_f64 * t1580 * t602 - 0.23005755572352449806e1_f64 * t1584 * t1586 - 0.47667319935800568892e0_f64 * t530 * t1590 + 0.47667319935800568892e0_f64 * t536 * t541 + 0.35750489951850426669e0_f64 * t1596 * t193 - 0.71500979903700853338e0_f64 * t1599 * t558;
    t1602
}
