//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 650/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk650<F: Float>(t4505: F, t965: F, t8345: F, t91: F, t20098: F, t24: F, t469: F, t20044: F, t464: F, t463: F, t20113: F, t8270: F, t1587: F, t3149: F, t4495: F, t20022: F, t8276: F) -> (F, F, F, F, F, F, F) {
    let t20329 = t4505 * t965;
    let t20331 = t91 * t8345 * t20329;
    let t20334 = t24 * t469 * t20098;
    let t20336 = t464 * t20044;
    let t20337 = t463 * t20336;
    let t20341 = t24 * t8270 * t20113;
    let t20345 = t1587 * t3149 * t4495;
    let t20348 = t8276 * t20022;
    (t20331, t20334, t20336, t20337, t20341, t20345, t20348)
}
