//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 623/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk623(t1713: f64, t372: f64, t1095: f64, t1426: f64, t175: f64, t5645: f64, t1008: f64, t1861: f64, t1089: f64, t1859: f64, t429: f64, t1298: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5944 = t1713 * t372;
    let t5946 = t1426 * t1095 * t5944;
    let t5950 = t1426 * t175 * t5645;
    let t5953 = t1008 * t1861;
    let t5956 = t1089 * t429 * t1859;
    let t5959 = t1298 * t506;
    (t5944, t5946, t5950, t5953, t5956, t5959)
}
