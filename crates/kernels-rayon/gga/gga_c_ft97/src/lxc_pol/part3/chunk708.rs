//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 708/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk708(t12310: f64, t12327: f64, t12356: f64, t12365: f64, t157: f64, t526: f64, t3421: f64, t8392: f64, t1045: f64, t2101: f64, t1055: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13102 = 4.0_f64 / 27.0_f64 * t12310;
    let t13108 = 2.0_f64 / 9.0_f64 * t12327;
    let t13117 = 4.0_f64 / 3.0_f64 * t12356;
    let t13120 = 2.0_f64 / 3.0_f64 * t12365;
    let t13140 = t526 * t157;
    let t13152 = 2.0_f64 / 27.0_f64 * t8392 * t3421;
    let t13153 = t2101 * t1045;
    let t13187 = t8232 * t1055;
    (t13102, t13108, t13117, t13120, t13140, t13152, t13153, t13187)
}
