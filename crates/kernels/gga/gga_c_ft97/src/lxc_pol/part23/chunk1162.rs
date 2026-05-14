//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1162/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1162<F: Float>(t28963: F, t6210: F, t1466: F, t28977: F, t681: F, t28971: F, t28862: F, t28869: F, t458: F, t6962: F, t6219: F, t29416: F, t6213: F, t2399: F, t7023: F, t6967: F, t98388: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t112512 = 2.0 / 9.0 * t6210 * t28963;
    let t112515 = 2.0 / 9.0 * t1466 * t681 * t28977;
    let t112520 = 2.0 / 9.0 * t1466 * t681 * t28971;
    let t112549 = t1466 * t681 * t28862 / 9.0;
    let t112565 = 2.0 / 9.0 * t1466 * t681 * t28869;
    let t112566 = t6962 * t458;
    let t112568 = t112566 * t6219 / 27.0;
    let t112602 = t29416 * t6213 / 9.0;
    let t112630 = t1466 * t2399 * t7023;
    let t112641 = t98388 * t6967 / 27.0;
    (t112512, t112515, t112520, t112549, t112565, t112566, t112568, t112602, t112630, t112641)
}
