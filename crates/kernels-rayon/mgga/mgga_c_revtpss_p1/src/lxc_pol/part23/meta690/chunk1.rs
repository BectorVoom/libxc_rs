//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2433/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2433(t560: f64, t9655: f64, t225: f64, t3896: f64, t39515: f64, t3900: f64, t9292: f64, t1419: f64, t9646: f64, t9648: f64, t1362: f64, t1363: f64, t39497: f64) -> (f64, f64, f64, f64, f64) {
    let t46361 = 1.0_f64 / t9655 / t560;
    let t46362 = t225 * t46361;
    let t46368 = 0.11564373972601816912e-1_f64 * t39515 * t3896;
    let t46369 = t9292 * t3900;
    let t46378 = t9646 * t1419 * t9648;
    let t46385 = 0.10118827226026589797e0_f64 * t1362 * t1363 * t39497;
    (t46362, t46368, t46369, t46378, t46385)
}
