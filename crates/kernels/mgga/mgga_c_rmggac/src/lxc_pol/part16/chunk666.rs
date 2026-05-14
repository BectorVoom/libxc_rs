//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 666/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk666<F: Float>(t10414: F, t10480: F, t82: F, t72: F, t10458: F, t515: F, t235: F, t10067: F, t10073: F, t10079: F, t10417: F, t884: F, t10420: F, t10085: F, t10091: F, t10096: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10481 = t10414 + t10480;
    let t10482 = t82 * t10481;
    let t10483 = t72 * t10482;
    let t10484 = t515 * t10458;
    let t10485 = t235 * t10484;
    let t10486 = 0.19957069503106347607e-1 * t10485;
    let t10488 = 0.212822999466489197e-4 * t10067;
    let t10490 = 0.1702583995731913576e-4 * t10073;
    let t10491 = 0.5107751987195740728e-4 * t10079;
    let t10492 = t884 * t10417;
    let t10493 = 0.11974241701863808564e0 * t10492;
    let t10494 = t884 * t10420;
    let t10495 = 0.59871208509319042821e-1 * t10494;
    let t10497 = 0.5107751987195740728e-4 * t10085;
    let t10498 = 0.5107751987195740728e-4 * t10091;
    let t10499 = 0.1702583995731913576e-4 * t10096;
    (t10481, t10482, t10483, t10484, t10486, t10488, t10490, t10491, t10493, t10495, t10497, t10498, t10499)
}
