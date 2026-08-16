//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1004/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1004<F: Float>(t14334: F, t14340: F, t1456: F, t1457: F, t1572: F, t1646: F, t204: F, t2476: F, t2843: F, t2877: F, t46606: F, t46609: F, t46612: F, t46614: F, t46630: F, t46633: F, t46635: F, t46638: F, t46642: F, t47964: F, t48087: F, t4950: F, t49862: F, t49866: F, t49874: F, t49878: F, t49917: F, t50596: F, t528: F, t557: F, t6963: F, t6964: F) -> F {
    let t50750 = F::cast_from(0.14300195980740170668e1_f64) * t4950 * t14340 + F::cast_from(0.14300195980740170668e1_f64) * t1572 * t1457 * t49862 + F::cast_from(0.14300195980740170668e1_f64) * t1572 * t1457 * t49866 - F::cast_from(0.35750489951850426669e0_f64) * t528 * t14334 * t1646 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t1457 * t49917 - F::cast_from(0.10725146985555128001e1_f64) * t557 * t1457 * t49878 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t49874 - t46606 + F::cast_from(0.92023022289409799224e1_f64) * t2476 * t204 * t50596 + t46609 + F::cast_from(0.71500979903700853338e0_f64) * t47964 * t2877 + F::cast_from(0.21450293971110256002e1_f64) * t48087 * t2843 - F::cast_from(0.14300195980740170668e1_f64) * t6963 * t6964 * t50596 + t46612 - t46614 + t46630 + t46633 - t46635 - t46638 + t46642;
    t50750
}
