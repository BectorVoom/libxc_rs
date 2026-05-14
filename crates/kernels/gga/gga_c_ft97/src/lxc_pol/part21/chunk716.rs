//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 716/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk716<F: Float>(t1053: F, t3565: F, t2179: F, t144: F, t4823: F, t9419: F, t3408: F, t920: F, t2222: F, t2221: F, t17151: F, t17155: F, t17158: F, t17161: F, t17165: F, t17170: F, t17174: F, t17178: F, t1901: F, t446: F, t9270: F, t9272: F, t9298: F, t9321: F) -> (F, F, F, F) {
    let t17181 = t1053 * t3565;
    let t17182 = t2179 * t17181;
    let t17183 = t144 * t17182;
    let t17186 = t9419 * t4823;
    let t17189 = t920 * t3408;
    let t17190 = t2222 * t17189;
    let t17191 = t2221 * t17190;
    let t17194 = -4.0 / 27.0 * t9270 - 4.0 / 27.0 * t9272 - 4.0 / 81.0 * t9298 - 2.0 / 3.0 * t446 * t17151 - t446 * t17155 / 3.0 + 2.0 / 9.0 * t1901 * t17158 + 4.0 / 9.0 * t1901 * t17161 - 4.0 / 27.0 * t1901 * t17165 + 4.0 / 27.0 * t9321 - 2.0 / 3.0 * t446 * t17170 - t446 * t17174 / 3.0 - t446 * t17178 / 3.0 + 4.0 / 3.0 * t446 * t17183 + 2.0 / 9.0 * t1901 * t17186 + 2.0 / 9.0 * t1901 * t17191;
    (t17181, t17189, t17190, t17194)
}
