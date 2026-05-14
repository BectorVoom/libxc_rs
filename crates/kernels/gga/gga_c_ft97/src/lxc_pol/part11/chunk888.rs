//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 888/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk888<F: Float>(t597: F, t9438: F, t9440: F, t9276: F, t9288: F, t39673: F, t39662: F, t39666: F, t39670: F, t39677: F, t39679: F, t39681: F, t39683: F, t39685: F, t39687: F, t39689: F, t39691: F, t39696: F, t39700: F, t39704: F) -> (F, F, F) {
    let t40591 = t597 * t9438;
    let t40592 = t40591 * t9440;
    let t40594 = t9276 * t9288;
    let t40599 = 280.0 / 243.0 * t39673;
    let t40611 = 8.0 / 9.0 * t39662 - 8.0 / 3.0 * t39666 - 8.0 / 3.0 * t39670 + t40599 - t39677 / 3.0 - 20.0 / 27.0 * t39679 + 8.0 / 9.0 * t39681 - 4.0 / 9.0 * t39683 - 8.0 / 9.0 * t39685 + 8.0 / 27.0 * t39687 - 8.0 / 27.0 * t39689 + 8.0 / 9.0 * t39691 - 8.0 / 9.0 * t39696 - 16.0 / 9.0 * t39700 + 3.0 / 4.0 * t39704;
    (t40592, t40594, t40611)
}
