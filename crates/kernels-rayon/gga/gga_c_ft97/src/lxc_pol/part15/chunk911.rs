//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 911/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk911(t4768: f64, t8282: f64, t1771: f64, t4776: f64, t4772: f64, t62246: f64, t62287: f64, t62309: f64, t62317: f64, t4743: f64, t8232: f64, t4819: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t62669 = t8282 * t4768;
    let t62745 = t1771 * t4776;
    let t62751 = t1771 * t4772;
    let t62822 = 4.0_f64 / 9.0_f64 * t62246;
    let t62846 = 4.0_f64 / 27.0_f64 * t62287;
    let t62853 = 8.0_f64 / 81.0_f64 * t62309;
    let t62856 = 8.0_f64 / 27.0_f64 * t62317;
    let t63120 = t8232 * t4743;
    let t63157 = t8232 * t4819;
    (t62669, t62745, t62751, t62822, t62846, t62853, t62856, t63120, t63157)
}
