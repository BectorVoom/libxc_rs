//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 780/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk780(t3103: f64, t942: f64, t110: f64, t1871: f64, t376: f64, t4547: f64, t89: f64, t4495: f64, t452: f64, t499: f64, t15885: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t16120 = t942 * t3103;
    let t16122 = t1871 * t110 * t16120;
    let t16126 = t89 * t376 * t4547;
    let t16129 = t452 * t499 * t4495;
    let t16133 = t452 * t110 * t15885;
    let t16137 = t452 * t986 * t3103;
    (t16122, t16126, t16129, t16133, t16137)
}
