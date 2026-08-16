//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 598/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk598(t4973: f64, t683: f64, t92: f64, t2401: f64, t3738: f64, t4967: f64, t4971: f64) -> (f64, f64, f64) {
    let t4974 = t683 * t4973;
    let t4975 = t92 * t4974;
    let t4977 = t2401 + 2.0_f64 / 9.0_f64 * t3738 - 2.0_f64 / 9.0_f64 * t4967 + 2.0_f64 / 3.0_f64 * t4971 - t4975 / 3.0_f64;
    (t4974, t4975, t4977)
}
