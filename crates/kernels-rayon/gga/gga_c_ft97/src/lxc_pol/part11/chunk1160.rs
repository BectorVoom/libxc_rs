//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1160/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1160(t10491: f64, t871: f64, t870: f64, t9577: f64, t2770: f64, t2832: f64, t10443: f64, t10509: f64, t10516: f64, t10763: f64, t1901: f64, t2405: f64, t2857: f64, t2867: f64, t2874: f64, t2877: f64, t319: f64, t4139: f64, t41691: f64, t41698: f64, t41718: f64, t4265: f64, t44210: f64, t44219: f64, t44518: f64, t44523: f64, t446: f64, t684: f64, t835: f64, t882: f64, t9587: f64) -> f64 {
    let t44528 = t10491 * t871;
    let t44533 = t870 * t9577;
    let t44538 = t2770 * t2832;
    let t44549 = 8.0_f64 / 3.0_f64 * t446 * t835 * t319 * t41691 - 8.0_f64 / 3.0_f64 * t446 * t835 * t882 * t9587 - 8.0_f64 / 3.0_f64 * t446 * t2857 * t319 * t41718 + 2.0_f64 / 3.0_f64 * t446 * t835 * t319 * t41698 - 8.0_f64 / 9.0_f64 * t1901 * t44518 * t2867 * t2405 + 8.0_f64 / 3.0_f64 * t1901 * t44523 * t10763 * t684 + 8.0_f64 / 3.0_f64 * t1901 * t44528 * t10516 * t684 - 16.0_f64 / 9.0_f64 * t1901 * t4139 * t44533 * t44210 + 4.0_f64 / 3.0_f64 * t1901 * t44538 * t2877 - 8.0_f64 / 3.0_f64 * t1901 * t10443 * t10509 - 4.0_f64 / 3.0_f64 * t1901 * t2874 * t4265 * t44219;
    t44549
}
