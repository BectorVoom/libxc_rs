//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2533/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2533(t14192: f64, t2782: f64, t46469: f64, t9994: f64, t544: f64, t9989: f64, t4003: f64, t215: f64, t268: f64, t4056: f64, t4101: f64, t543: f64) -> (f64, f64, f64, f64) {
    let t46472 = t2782 * t14192 * t46469 * t9994;
    let t46475 = 1.0_f64 / t9989 / t544;
    let t46478 = t4003 * t4003;
    let t46490 = t4101 * t268 * t215 * t4056 * t543;
    (t46472, t46475, t46478, t46490)
}
