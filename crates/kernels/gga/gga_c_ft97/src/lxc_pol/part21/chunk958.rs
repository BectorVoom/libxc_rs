//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 958/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk958<F: Float>(t86: F, t29612: F, t30020: F, t113: F, t1342: F, t4635: F, t5: F, t6570: F, t992: F, t1058: F, t6616: F, t28: F, t1701: F, t4698: F, t5546: F, t4710: F, t4702: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t30021 = t29612 + t30020;
    let t30032 = piecewise3(t87, 0.0, t5 * t30021 * t113 / 4.0 + t5 * t6570 * t992 / 2.0 + t5 * t1342 * t4635 / 4.0);
    let t30033 = t6616 * t1058;
    let t30034 = t28 * t30033;
    let t30038 = t1701 * t5546 * t4698;
    let t30042 = t1701 * t5546 * t4710;
    let t30058 = t72 * t4702;
    (t30021, t30032, t30033, t30034, t30038, t30042, t30058)
}
