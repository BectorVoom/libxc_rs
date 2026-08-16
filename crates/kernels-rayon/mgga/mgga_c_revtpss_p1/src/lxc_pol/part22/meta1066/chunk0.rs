//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3816/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3816(t1317: f64, t22195: f64, t48235: f64, t48237: f64, t48240: f64, t48243: f64, t46975: f64, t46977: f64, t46983: f64, t1320: f64, t22193: f64, t10186: f64, t198: f64, t39531: f64, t49544: f64, t6836: f64, t6930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73360 = t1317 * t22195;
    let t73361 = 8.0_f64 * t73360;
    let t73364 = 80.0_f64 * t48235;
    let t73365 = 8.0_f64 * t48237;
    let t73366 = 4.0_f64 * t48240;
    let t73367 = 2.0_f64 * t48243;
    let t73371 = 160.0_f64 * t46975;
    let t73372 = 240.0_f64 * t46977;
    let t73373 = 8.0_f64 * t46983;
    let t73374 = t1320 * t22193;
    let t73375 = 8.0_f64 * t73374;
    let t73376 = 6.0_f64 * t10186 * t198 * t6836 + 12.0_f64 * t49544 * t6930 + t39531 + t73361 + t73364 - t73365 + t73366 + t73367 - t73371 - t73372 - t73373 - t73375;
    (t73361, t73364, t73365, t73366, t73367, t73371, t73372, t73373, t73375, t73376)
}
