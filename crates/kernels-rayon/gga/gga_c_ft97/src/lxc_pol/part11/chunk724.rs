//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 724/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk724(t2526: f64, t766: f64, t2568: f64, t242: f64, t1901: f64, t446: f64, t9788: f64, t9794: f64, t9799: f64, t9805: f64, t9810: f64, t9813: f64, t9816: f64, t9819: f64, t9822: f64, t9824: f64, t9826: f64, t9828: f64, t9831: f64, t9835: f64) -> (f64, f64, f64, f64) {
    let t9838 = t766 * t2526;
    let t9839 = t2568 * t9838;
    let t9840 = t242 * t9839;
    let t9843 = 2.0_f64 / 3.0_f64 * t1901 * t9788 - 2.0_f64 / 3.0_f64 * t1901 * t9794 - 2.0_f64 / 3.0_f64 * t1901 * t9799 + 2.0_f64 / 9.0_f64 * t1901 * t9805 + 2.0_f64 / 9.0_f64 * t1901 * t9810 + 2.0_f64 / 3.0_f64 * t9813 - t446 * t9816 - t446 * t9819 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t9822 - 4.0_f64 / 9.0_f64 * t9824 + t9826 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t9828 - 2.0_f64 / 3.0_f64 * t446 * t9831 + 4.0_f64 / 9.0_f64 * t446 * t9835 + 2.0_f64 * t446 * t9840;
    (t9838, t9839, t9840, t9843)
}
