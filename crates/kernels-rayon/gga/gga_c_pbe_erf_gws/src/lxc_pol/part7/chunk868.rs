//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 868/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk868(t4929: f64, t5211: f64, t617: f64, t7116: f64, t4882: f64, t5213: f64, t5210: f64, t735: f64, t5214: f64, t1403: f64) -> (f64, f64, f64, f64) {
    let t16662 = 64.0_f64 / 15.0_f64 * t5211 * t7116 * t617 * t4929;
    let t16665 = 64.0_f64 / 15.0_f64 * t5211 * t5213 * t4882;
    let t16666 = t5210 * t735;
    let t16667 = t16666 * t5214;
    let t16668 = 128.0_f64 / 45.0_f64 * t16667;
    let t16669 = t1403 * t1403;
    (t16662, t16665, t16668, t16669)
}
