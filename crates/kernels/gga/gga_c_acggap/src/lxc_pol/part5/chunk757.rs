//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 757/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk757<F: Float>(t6293: F, t960: F, t1866: F, t3282: F, t1567: F, t513: F, t1524: F, t530: F, t1782: F, t435: F, t301: F, t1879: F, t372: F, t1150: F, t1173: F, t335: F, t367: F, t3671: F, t3673: F, t3677: F, t3679: F, t3694: F, t3699: F, t3702: F, t5169: F, t5175: F, t6271: F, t6279: F, t6283: F, t6286: F, t6290: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6294 = t960 * t6293;
    let t6297 = t3282 * t1866;
    let t6300 = t1567 * t513;
    let t6301 = t960 * t6300;
    let t6304 = t530 * t1524;
    let t6305 = t960 * t6304;
    let t6308 = t435 * t1782;
    let t6309 = t6308 * t301;
    let t6310 = t960 * t6309;
    let t6313 = t1879 * t372;
    let t6314 = t960 * t6313;
    let t6317 = 0.34299214494455789578e-2 * t1173 * t6271 - t5169 - t5175 - 0.22675591804667994221e-1 * t3671 + 0.11337795902333997111e-1 * t3673 - 0.11337795902333997111e-1 * t3677 + 0.80031500487063509016e-2 * t3679 - t3694 - t3699 - t3702 + t367 * t6279 / 48.0 + t1150 * t6283 / 16.0 - 7.0 / 48.0 * t6286 + t335 * t6290 / 48.0 + t367 * t6294 / 48.0 + t335 * t6297 / 24.0 + t335 * t6301 / 24.0 + t335 * t6305 / 24.0 - t335 * t6310 / 24.0 - t367 * t6314 / 16.0;
    (t6294, t6297, t6300, t6301, t6304, t6305, t6308, t6309, t6310, t6313, t6314, t6317)
}
