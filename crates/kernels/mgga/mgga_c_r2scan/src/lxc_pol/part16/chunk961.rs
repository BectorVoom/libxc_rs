//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 961/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk961<F: Float>(t20146: F, t37943: F, t37945: F, t37942: F, t565: F, t19791: F, t1576: F, t546: F, t25851: F, t512: F, t10757: F, t776: F, t2111: F, t2164: F, t22766: F, t20450: F, t2215: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37947 = t37943 * t37945 * t20146;
    let t37949 = t565 * t37942;
    let t37951 = t37949 * t37945 * t19791;
    let t37961 = t565 * t1576;
    let t37965 = t546 * t1576;
    let t37982 = t512 * t25851;
    let t37985 = t776 * t10757;
    let t38001 = t2111 * t22766 * t2164;
    let t38002 = 0.1590300183910403919e-2 * t38001;
    let t38003 = t20450 * t2215;
    (t37947, t37949, t37951, t37961, t37965, t37982, t37985, t38002, t38003)
}
