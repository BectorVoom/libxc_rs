//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1077/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1077<F: Float>(t20146: F, t37943: F, t37945: F, t37942: F, t565: F, t19791: F, t10708: F, t10710: F, t20132: F, t10728: F, t20102: F, t1576: F) -> (F, F, F, F, F, F) {
    let t37947 = t37943 * t37945 * t20146;
    let t37949 = t565 * t37942;
    let t37951 = t37949 * t37945 * t19791;
    let t37954 = t10708 * t10710 * t20132;
    let t37957 = t10728 * t10710 * t20102;
    let t37961 = t565 * t1576;
    (t37947, t37949, t37951, t37954, t37957, t37961)
}
