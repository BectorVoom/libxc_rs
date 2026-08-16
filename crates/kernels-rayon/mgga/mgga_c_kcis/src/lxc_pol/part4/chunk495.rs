//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 495/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk495(t609: f64, t1928: f64, t20: f64, t610: f64, t1601: f64, t1650: f64, t1600: f64, t2093: f64) -> (f64, f64, f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t2095 = t1928 * t20;
    let t2096 = t610 * t2095;
    let t2099 = t1601 * t1650;
    let t2100 = t1600 * t2099;
    let t2104 = piecewise3(t614, t2093, -t2093);
    (t2095, t2096, t2099, t2100, t2104)
}
