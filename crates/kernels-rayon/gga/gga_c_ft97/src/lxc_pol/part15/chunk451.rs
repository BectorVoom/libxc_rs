//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 451/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk451(t1902: f64, t4607: f64, t920: f64, t979: f64, t1910: f64, t1909: f64, t110: f64, t4458: f64, t447: f64, t1887: f64, t1901: f64, t28: f64, t3177: f64, t3224: f64, t3260: f64, t3286: f64, t446: f64, t4547: f64, t4553: f64, t4557: f64, t4561: f64, t4565: f64, t4569: f64, t4574: f64, t4591: f64, t4595: f64, t4599: f64, t4603: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4608 = t1902 * t4607;
    let t4611 = t920 * t979;
    let t4612 = t1910 * t4611;
    let t4613 = t1909 * t4612;
    let t4617 = t447 * t110 * t4458;
    let t4621 = t89 * t28 * t4547 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t4553 - 2.0_f64 / 9.0_f64 * t446 * t4557 - t446 * t4561 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t4565 + 2.0_f64 / 3.0_f64 * t446 * t4569 + 2.0_f64 / 3.0_f64 * t446 * t4574 + 2.0_f64 / 9.0_f64 * t3224 + 2.0_f64 / 9.0_f64 * t3260 + t1887 - 2.0_f64 / 9.0_f64 * t3177 - t446 * t4591 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t4595 - 2.0_f64 / 3.0_f64 * t446 * t4599 - t446 * t4603 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t4608 + 2.0_f64 / 9.0_f64 * t1901 * t4613 + 2.0_f64 / 9.0_f64 * t446 * t4617 + 2.0_f64 / 27.0_f64 * t3286;
    (t4608, t4611, t4612, t4613, t4617, t4621)
}
