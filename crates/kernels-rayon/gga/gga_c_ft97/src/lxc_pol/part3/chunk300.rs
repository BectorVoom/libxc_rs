//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 300/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk300(t1234: f64, t845: f64, t91: f64, t1188: f64, t1215: f64, t860: f64) -> (f64, f64) {
    let t1236 = t91 * t845 * t1234;
    let t1240 = t1236 / 6.0_f64 - t860 - t1188 / 9.0_f64 - t1215 / 3.0_f64;
    (t1236, t1240)
}
