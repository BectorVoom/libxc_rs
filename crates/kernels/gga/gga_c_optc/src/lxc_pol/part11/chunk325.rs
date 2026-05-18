//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 325/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk325<F: Float>(t1382: F, t287: F, t297: F, t914: F, t1378: F, t312: F, t894: F, t1389: F, t913: F, t927: F, t930: F, t940: F, t951: F, t953: F) -> (F, F, F) {
    let t1396 = t287 * t1382;
    let t1397 = t1396 * t297;
    let t1398 = t914 * t1397;
    let t1401 = t914 * t1378;
    let t1404 = t312 * t1382;
    let t1405 = t1404 * t297;
    let t1406 = t894 * t1405;
    let t1411 = F::new(0.11360101276506094136e1) * t913 * t1398 + t927 + F::new(0.28977204965962526182e-1) * t930 * t1401 + F::new(0.5848048239485271795e1) * t940 * t1406 + t951 + F::new(0.50380704458364197288e-2) * t953 * t1389;
    (t1397, t1405, t1411)
}
