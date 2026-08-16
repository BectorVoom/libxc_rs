//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 900/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk900(t13943: f64, t2606: f64, t3870: f64, t9787: f64, t11593: f64, t13894: f64, t13899: f64, t13903: f64, t13905: f64, t13907: f64, t13911: f64, t13915: f64, t13919: f64, t13924: f64, t13929: f64, t13933: f64, t13935: f64, t13939: f64, t1901: f64, t446: f64) -> f64 {
    let t13944 = t2606 * t13943;
    let t13947 = t9787 * t3870;
    let t13950 = -4.0_f64 / 9.0_f64 * t11593 * t13894 - 4.0_f64 / 9.0_f64 * t11593 * t13899 + t13903 + t13905 + 2.0_f64 / 3.0_f64 * t446 * t13907 - 2.0_f64 / 3.0_f64 * t446 * t13911 - t446 * t13915 / 3.0_f64 - t446 * t13919 / 3.0_f64 + t446 * t13924 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t13929 + t13933 + t1901 * t13935 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t1901 * t13939 + 2.0_f64 / 9.0_f64 * t1901 * t13944 + 2.0_f64 / 9.0_f64 * t1901 * t13947;
    t13950
}
