//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1076/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1076(t20146: f64, t37943: f64, t37945: f64, t37942: f64, t565: f64, t19791: f64, t10708: f64, t10710: f64, t20132: f64, t10728: f64, t20102: f64, t1576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37947 = t37943 * t37945 * t20146;
    let t37949 = t565 * t37942;
    let t37951 = t37949 * t37945 * t19791;
    let t37954 = t10708 * t10710 * t20132;
    let t37957 = t10728 * t10710 * t20102;
    let t37961 = t565 * t1576;
    (t37947, t37949, t37951, t37954, t37957, t37961)
}
