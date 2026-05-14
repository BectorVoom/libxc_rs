//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1371/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1371<F: Float>(t1701: F, t27494: F, t4088: F, t2035: F, t31515: F, t811: F, t820: F, t127429: F, t4092: F, t123362: F, t420: F, t2725: F, t30674: F, t1200: F, t123503: F, t31465: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127519 = t1701 * t27494 * t4088;
    let t127530 = t2035 * t31515 * t811;
    let t127534 = t2035 * t31515 * t820;
    let t127537 = t4092 * t127429;
    let t127539 = t420 * t123362 * t811;
    let t127542 = t2725 * t30674;
    let t127543 = t1200 * t127542;
    let t127545 = t420 * t123362 * t820;
    let t127548 = t31465 * t123503;
    (t127519, t127530, t127534, t127537, t127539, t127542, t127543, t127545, t127548)
}
