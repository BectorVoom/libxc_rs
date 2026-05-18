//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 365/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk365<F: Float>(t1549: F, t1552: F, t1556: F, t1562: F, t1567: F, t1572: F, t1573: F, t1577: F, t1580: F, t1584: F, t1586: F, t1590: F, t1596: F, t1599: F, t193: F, t530: F, t532: F, t536: F, t541: F, t558: F, t574: F, t602: F) -> F {
    let t1602 = F::new(0.71500979903700853338e0) * t1549 * t193 - F::new(0.35750489951850426669e0) * t530 * t1552 - F::new(0.71500979903700853338e0) * t1556 * t532 - F::new(0.69017266717057349418e1) * t1562 * t1567 + F::new(0.71500979903700853338e0) * t1572 * t1573 - F::new(0.46011511144704899612e1) * t574 * t1577 + F::new(0.46011511144704899612e1) * t1580 * t602 - F::new(0.23005755572352449806e1) * t1584 * t1586 - F::new(0.47667319935800568892e0) * t530 * t1590 + F::new(0.47667319935800568892e0) * t536 * t541 + F::new(0.35750489951850426669e0) * t1596 * t193 - F::new(0.71500979903700853338e0) * t1599 * t558;
    t1602
}
