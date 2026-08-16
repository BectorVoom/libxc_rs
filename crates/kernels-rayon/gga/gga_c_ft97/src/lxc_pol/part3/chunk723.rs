//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 723/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk723(t2492: f64, t265: f64, t9802: f64, t1882: f64, t3983: f64, t3839: f64, t1140: f64, t8232: f64, t3848: f64, t1170: f64, t3953: f64, t681: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14212 = 2.0_f64 / 9.0_f64 * t1882 * t3983;
    let t14223 = 4.0_f64 / 9.0_f64 * t1882 * t3839;
    let t14224 = t8232 * t1140;
    let t14232 = 2.0_f64 / 27.0_f64 * t1882 * t3848;
    let t14233 = t8232 * t1170;
    let t14240 = 2.0_f64 / 9.0_f64 * t89 * t681 * t3953;
    (t14196, t14200, t14212, t14223, t14224, t14232, t14233, t14240)
}
