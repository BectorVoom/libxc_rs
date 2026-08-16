//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2527/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2527(t10153: f64, t2435: f64, t2439: f64, t3895: f64, t4078: f64, t39552: f64, t562: f64, t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64) -> (f64, f64, f64, f64, f64) {
    let t46353 = t2435 * t10153;
    let t46356 = t2439 * t3895 * t4078;
    let t46359 = 0.88356352675825229576e-3_f64 * t39552 * t562;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t46362 = t225 * t46361;
    let t46368 = 0.11564373972601816912e-1_f64 * t39515 * t3896;
    (t46353, t46356, t46359, t46362, t46368)
}
