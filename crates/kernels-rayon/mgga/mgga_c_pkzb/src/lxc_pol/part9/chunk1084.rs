//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1084/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1084(t17402: f64, t218: f64, t5555: f64, t679: f64, t1878: f64, t1885: f64, t1889: f64, t5568: f64, t675: f64, t5572: f64, t16194: f64, t213: f64, t778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17403 = 0.13490888888888888889e1_f64 * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17408 = t218 * t1878 * t1885;
    let t17411 = t218 * t1878 * t1889;
    let t17414 = t218 * t675 * t5568;
    let t17417 = t218 * t675 * t5572;
    let t17432 = 1.0_f64 / t213 / t16194 / t778 / 96.0_f64;
    (t17403, t17405, t17408, t17411, t17414, t17417, t17432)
}
