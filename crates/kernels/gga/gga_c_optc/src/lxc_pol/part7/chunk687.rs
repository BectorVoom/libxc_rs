//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 687/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk687<F: Float>(t1789: F, t508: F, t1793: F, t209: F, t110: F, t514: F, t535: F, t1756: F, t1759: F, t580: F, t1864: F, t565: F) -> (F, F, F, F, F, F, F, F) {
    let t6481 = t508 * t1789;
    let t6484 = F::new(0.85917146441092277512e0) * t209 * t6481 * t1793;
    let t6485 = t110 * t514;
    let t6488 = F::new(0.71233333333333333334e-1) * t209 * t6485 * t535;
    let t6492 = F::new(0.10685e0) * t209 * t508 * t1756 * t1759;
    let t6493 = t110 * t580;
    let t6500 = t508 * t1864;
    let t6504 = t110 * t565;
    (t6481, t6484, t6485, t6488, t6492, t6493, t6500, t6504)
}
