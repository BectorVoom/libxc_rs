//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 811/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk811<F: Float>(t25528: F, t5508: F, t28: F, t432: F, t984: F, t5507: F, t1308: F, t3289: F, t376: F, t6456: F, t1286: F, t6460: F, t497: F, t6455: F, t6413: F, t92: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25529 = t25528 * t5508;
    let t25530 = t28 * t25529;
    let t25533 = t984 * t432;
    let t25534 = t5507 * t25533;
    let t25535 = t28 * t25534;
    let t25538 = t1308 * t3289;
    let t25539 = t28 * t25538;
    let t25542 = t376 * t6456;
    let t25543 = t1286 * t25542;
    let t25545 = t376 * t6460;
    let t25546 = t1286 * t25545;
    let t25552 = t6455 * t497;
    let t25553 = t28 * t25552;
    let t25558 = t6413 * t92;
    (t25529, t25530, t25533, t25534, t25535, t25538, t25539, t25542, t25543, t25545, t25546, t25552, t25553, t25558)
}
