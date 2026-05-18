//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 354/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk354<F: Float>(t1576: F, t1603: F, t1184: F, t1206: F, t1214: F, t1222: F, t1229: F, t1240: F, t1574: F, t1578: F, t1581: F, t1584: F, t1588: F, t1592: F, t1596: F, t1600: F, t434: F, t469: F) -> F {
    let t1604 = t1576 * t1603;
    let t1607 = -F::new(0.1013812832824605378e-3) * t1574 * t1578 - F::new(0.6951859425083008306e-4) * t1581 * t469 - F::new(0.20855578275249024918e-2) * t434 * t1584 - F::new(0.10427789137624512459e-2) * t434 * t1588 + t1222 - t1184 + t1229 - t1240 - F::new(0.12360406057797588768e-3) * t1592 * t1596 + t1206 + t1214 - F::new(0.1013812832824605378e-3) * t1600 * t1604;
    t1607
}
