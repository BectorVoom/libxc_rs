//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1583/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1583(t1432: f64, t22964: f64, t686: f64, t72: f64, t14239: f64, t22332: f64, t10023: f64, t22863: f64, t14141: f64, t23037: f64, t22336: f64, t13790: f64, t6843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86374 = t1432 * t22964 * t72 * t686;
    let t86377 = t14239 * t22332;
    let t86381 = t10023 * t22863 * t72 * t686;
    let t86401 = t14141 * t23037 * t72 * t686;
    let t86411 = t14239 * t22336;
    let t86413 = t13790 * t6843;
    (t86374, t86377, t86381, t86401, t86411, t86413)
}
