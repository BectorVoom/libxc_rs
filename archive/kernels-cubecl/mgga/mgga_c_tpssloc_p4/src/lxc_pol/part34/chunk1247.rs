//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1247/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1247<F: Float>(t107634: F, t108844: F, t1390: F, t1774: F, t1849: F, t19451: F, t1983: F, t20356: F, t2039: F, t20698: F, t20702: F, t2079: F, t2094: F, t22425: F, t22574: F, t28821: F, t28830: F, t28951: F, t28952: F, t29201: F, t29211: F, t29214: F, t29219: F, t29241: F, t29243: F, t33899: F, t4028: F, t510: F, t574: F, t6287: F, t6468: F, t652: F, t7042: F, t7458: F, t7685: F, t7801: F, t7802: F, t7806: F, t7900: F, t7904: F) -> F {
    let t108856 = -F::cast_from(6.0_f64) * t19451 * t7806 - F::cast_from(6.0_f64) * t7042 * t20702 - F::cast_from(6.0_f64) * t7458 * t29214 - F::cast_from(6.0_f64) * t652 * t6287 * t7801 - F::cast_from(2.0_f64) * t652 * t510 * t107634 + F::cast_from(3.0_f64) * t7900 * t6468 + F::cast_from(6.0_f64) * t1983 * t20356 * t2094 * t1390 + F::cast_from(3.0_f64) * t29241 * t1849 - F::cast_from(6.0_f64) * t7458 * t28952 - F::cast_from(6.0_f64) * t652 * t1774 * t28951 - F::cast_from(12.0_f64) * t4028 * t29219 - F::cast_from(6.0_f64) * t19451 * t7802 - F::cast_from(18.0_f64) * t22574 * t33899 * t28830 - F::cast_from(6.0_f64) * t4028 * t29211 + t108844 * t574 + F::cast_from(6.0_f64) * t7685 * t29243 + F::cast_from(9.0_f64) * t28821 * t7904 - F::cast_from(6.0_f64) * t7685 * t29201 + t2079 * t20698 - F::cast_from(2.0_f64) * t652 * t22425 * t2039;
    t108856
}
