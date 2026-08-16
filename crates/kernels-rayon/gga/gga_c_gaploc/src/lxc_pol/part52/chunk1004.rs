//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1004/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1004(t14334: f64, t14340: f64, t1456: f64, t1457: f64, t1572: f64, t1646: f64, t204: f64, t2476: f64, t2843: f64, t2877: f64, t46606: f64, t46609: f64, t46612: f64, t46614: f64, t46630: f64, t46633: f64, t46635: f64, t46638: f64, t46642: f64, t47964: f64, t48087: f64, t4950: f64, t49862: f64, t49866: f64, t49874: f64, t49878: f64, t49917: f64, t50596: f64, t528: f64, t557: f64, t6963: f64, t6964: f64) -> f64 {
    let t50750 = 0.14300195980740170668e1_f64 * t4950 * t14340 + 0.14300195980740170668e1_f64 * t1572 * t1457 * t49862 + 0.14300195980740170668e1_f64 * t1572 * t1457 * t49866 - 0.35750489951850426669e0_f64 * t528 * t14334 * t1646 + 0.35750489951850426669e0_f64 * t1456 * t1457 * t49917 - 0.10725146985555128001e1_f64 * t557 * t1457 * t49878 + 0.71500979903700853338e0_f64 * t1572 * t1457 * t49874 - t46606 + 0.92023022289409799224e1_f64 * t2476 * t204 * t50596 + t46609 + 0.71500979903700853338e0_f64 * t47964 * t2877 + 0.21450293971110256002e1_f64 * t48087 * t2843 - 0.14300195980740170668e1_f64 * t6963 * t6964 * t50596 + t46612 - t46614 + t46630 + t46633 - t46635 - t46638 + t46642;
    t50750
}
