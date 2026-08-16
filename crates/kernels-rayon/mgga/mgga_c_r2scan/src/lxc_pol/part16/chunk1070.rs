//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1070/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1070(t20146: f64, t37943: f64, t37945: f64, t37942: f64, t565: f64, t19791: f64, t1576: f64, t546: f64, t25851: f64, t512: f64, t10757: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37947 = t37943 * t37945 * t20146;
    let t37949 = t565 * t37942;
    let t37951 = t37949 * t37945 * t19791;
    let t37961 = t565 * t1576;
    let t37965 = t546 * t1576;
    let t37982 = t512 * t25851;
    let t37985 = t776 * t10757;
    (t37947, t37949, t37951, t37961, t37965, t37982, t37985)
}
