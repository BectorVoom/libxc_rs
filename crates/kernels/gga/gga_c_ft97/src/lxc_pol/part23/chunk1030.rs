//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1030/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1030<F: Float>(t245: F, t30918: F, t31322: F, t1459: F, t21: F, t4431: F, t5: F, t6953: F, t920: F, t1091: F, t28985: F, t2665: F, t29024: F, t4973: F, t6217: F, t10409: F, t4965: F) -> (F, F, F, F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t31323 = t30918 + t31322;
    let t31334 = piecewise3(t246, 0.0, t5 * t31323 * t21 / 4.0 + t5 * t6953 * t920 / 2.0 + t5 * t1459 * t4431 / 4.0);
    let t31339 = t28985 * t1091;
    let t31340 = t2665 * t31339;
    let t31344 = t2665 * t29024 * t1091;
    let t31348 = t2665 * t6217 * t4973;
    let t31352 = t10409 * t6217 * t4965;
    (t31323, t31334, t31340, t31344, t31348, t31352)
}
