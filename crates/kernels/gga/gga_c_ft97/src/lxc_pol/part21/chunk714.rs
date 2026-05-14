//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 714/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk714<F: Float>(t1017: F, t3565: F, t574: F, t605: F, t1060: F, t3052: F, t569: F, t4462: F, t616: F, t15772: F, t167: F, t2205: F, t4454: F, t12963: F, t12965: F, t12967: F, t12975: F, t17101: F, t17104: F, t17107: F, t17111: F, t17115: F, t17120: F, t3281: F, t446: F) -> (F, F) {
    let t17123 = t1017 * t3565;
    let t17125 = t574 * t605 * t17123;
    let t17129 = t569 * t1060 * t3052;
    let t17133 = t569 * t616 * t4462;
    let t17137 = t569 * t167 * t15772;
    let t17141 = t2205 * t616 * t4454;
    let t17144 = 2.0 / 3.0 * t446 * t17101 - 2.0 / 27.0 * t17104 - 2.0 / 3.0 * t446 * t17107 - 2.0 / 9.0 * t446 * t17111 + 2.0 / 3.0 * t446 * t17115 + 2.0 / 3.0 * t446 * t17120 + 2.0 / 3.0 * t446 * t17125 - t12963 - t12965 - t12967 - t12975 - 4.0 / 9.0 * t3281 * t17129 - t446 * t17133 / 9.0 - t446 * t17137 / 9.0 - 2.0 / 27.0 * t446 * t17141;
    (t17123, t17144)
}
