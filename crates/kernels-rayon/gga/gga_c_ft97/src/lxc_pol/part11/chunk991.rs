//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 991/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk991(t597: f64, t9438: f64, t9440: f64, t9276: f64, t9288: f64, t39673: f64, t39662: f64, t39666: f64, t39670: f64, t39677: f64, t39679: f64, t39681: f64, t39683: f64, t39685: f64, t39687: f64, t39689: f64, t39691: f64, t39696: f64, t39700: f64, t39704: f64) -> (f64, f64, f64) {
    let t40591 = t597 * t9438;
    let t40592 = t40591 * t9440;
    let t40594 = t9276 * t9288;
    let t40599 = 280.0_f64 / 243.0_f64 * t39673;
    let t40611 = 8.0_f64 / 9.0_f64 * t39662 - 8.0_f64 / 3.0_f64 * t39666 - 8.0_f64 / 3.0_f64 * t39670 + t40599 - t39677 / 3.0_f64 - 20.0_f64 / 27.0_f64 * t39679 + 8.0_f64 / 9.0_f64 * t39681 - 4.0_f64 / 9.0_f64 * t39683 - 8.0_f64 / 9.0_f64 * t39685 + 8.0_f64 / 27.0_f64 * t39687 - 8.0_f64 / 27.0_f64 * t39689 + 8.0_f64 / 9.0_f64 * t39691 - 8.0_f64 / 9.0_f64 * t39696 - 16.0_f64 / 9.0_f64 * t39700 + 3.0_f64 / 4.0_f64 * t39704;
    (t40592, t40594, t40611)
}
