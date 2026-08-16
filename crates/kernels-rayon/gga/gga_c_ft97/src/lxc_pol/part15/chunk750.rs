//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 750/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk750(t21025: f64, t637: f64, t639: f64, t20044: f64, t632: f64, t72: f64, t1073: f64, t4872: f64, t8618: f64, t3640: f64, t4883: f64, t20022: f64, t8633: f64, t8634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21027 = t637 * t639 * t21025;
    let t21031 = t72 * t632 * t20044;
    let t21034 = t4872 * t1073;
    let t21036 = t637 * t8618 * t21034;
    let t21040 = t637 * t3640 * t4883;
    let t21044 = t8633 * t8634 * t20022;
    (t21027, t21031, t21034, t21036, t21040, t21044)
}
