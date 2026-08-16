//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1703/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1703(t4077: f64, t3896: f64, t39515: f64, t3900: f64, t9292: f64, t1419: f64, t9646: f64, t9648: f64, t10147: f64, t1357: f64, t689: f64, t1362: f64, t1363: f64, t39497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46363 = t4077 * t4077;
    let t46368 = 0.11564373972601816912e-1_f64 * t39515 * t3896;
    let t46369 = t9292 * t3900;
    let t46378 = t9646 * t1419 * t9648;
    let t46381 = t689 * t1357 * t10147;
    let t46385 = 0.10118827226026589797e0_f64 * t1362 * t1363 * t39497;
    (t46363, t46368, t46369, t46378, t46381, t46385)
}
