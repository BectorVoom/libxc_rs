//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 773/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk773<F: Float>(t8846: F, t8852: F, t8856: F, t8860: F, t8864: F, t8872: F, t9597: F, t9599: F, t8881: F, t8885: F, t9042: F, t9047: F, t9052: F, t9056: F, t9071: F, t9073: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42531 = 0.212822999466489197e-4 * t8846;
    let t42534 = 0.60975299583150056624e-3 * t8852;
    let t42535 = 0.60975299583150056624e-3 * t8856;
    let t42536 = 0.60975299583150056624e-3 * t8860;
    let t42537 = 0.60975299583150056624e-3 * t8864;
    let t42539 = 0.17961362552795712846e0 * t8872;
    let t42540 = 2.0 * t9597;
    let t42541 = 2.0 * t9599;
    let t42546 = 0.5987120850931904282e-1 * t8881;
    let t42547 = 0.17961362552795712846e0 * t8885;
    let t42549 = 0.1702583995731913576e-4 * t9042;
    let t42550 = 0.212822999466489197e-4 * t9047;
    let t42551 = 0.1702583995731913576e-4 * t9052;
    let t42552 = 0.5107751987195740728e-4 * t9056;
    let t42554 = 0.11974241701863808564e0 * t9071;
    let t42555 = 0.11974241701863808564e0 * t9073;
    (t42531, t42534, t42535, t42536, t42537, t42539, t42540, t42541, t42546, t42547, t42549, t42550, t42551, t42552, t42554, t42555)
}
