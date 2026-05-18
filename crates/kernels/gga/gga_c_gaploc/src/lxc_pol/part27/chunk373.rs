//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 373/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk373<F: Float>(t1604: F, t1605: F, t1609: F, t1613: F, t1617: F, t1621: F, t1625: F, t1629: F, t1632: F, t1635: F, t1638: F, t1641: F, t1644: F, t1646: F, t193: F, t557: F, t567: F, t571: F, t574: F, t576: F, t597: F) -> F {
    let t1649 = F::new(0.71500979903700853338e0) * t1604 * t1605 + F::new(0.23005755572352449806e1) * t597 * t1609 - F::new(0.23005755572352449806e1) * t574 * t1613 + F::new(0.23005755572352449806e1) * t1617 * t571 + F::new(0.11502877786176224903e1) * t567 * t1621 + F::new(0.35750489951850426669e0) * t1625 * t193 + F::new(0.61348681526273199483e1) * t597 * t1629 + F::new(0.30674340763136599742e1) * t567 * t1632 - F::new(0.61348681526273199483e1) * t574 * t1635 - F::new(0.47667319935800568892e0) * t557 * t1638 - F::new(0.46011511144704899612e1) * t1641 * t576 - F::new(0.71500979903700853338e0) * t1644 * t1646;
    t1649
}
