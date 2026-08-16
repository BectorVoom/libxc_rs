//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1668/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1668(t1811: f64, t5219: f64, t1828: f64, t5497: f64, t3737: f64, t1269: f64, t6628: f64, t3783: f64, t3769: f64, t1280: f64, t20703: f64, t1287: f64, t5284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21394 = t5219 * t1811;
    let t21407 = t1828 * t5497;
    let t21408 = t3737 * t21407;
    let t21415 = t1269 * t6628;
    let t21416 = t21415 * t3783;
    let t21427 = t21415 * t3769;
    let t21430 = t1280 * t20703;
    let t21436 = t1811 * t5284 * t1287;
    (t21394, t21408, t21416, t21427, t21430, t21436)
}
