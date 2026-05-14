//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 533/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk533<F: Float>(t1106: F, t1181: F, t530: F, t3361: F, t1111: F, t1165: F, t4267: F, t1562: F, t3431: F, t3360: F, t3402: F, t1101: F, t1470: F, t3409: F, t1410: F, t174: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4273 = t1181 * t530 * t1106;
    let t4275 = 0.34299214494455789578e-2 * t3361 * t4273;
    let t4277 = t1165 * t4267 * t1111;
    let t4279 = 0.34299214494455789578e-2 * t3361 * t4277;
    let t4280 = t3431 * t1562;
    let t4282 = t3360 * t3402;
    let t4284 = t1165 * t530 * t1101;
    let t4285 = t4282 * t4284;
    let t4288 = 0.40015750243531754508e-2 * t3409 * t1470;
    let t4289 = t174 * t1410;
    (t4273, t4275, t4277, t4279, t4280, t4284, t4285, t4288, t4289)
}
