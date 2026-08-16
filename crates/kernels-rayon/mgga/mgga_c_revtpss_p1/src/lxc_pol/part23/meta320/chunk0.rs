//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1609/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1609(t13632: f64, t512: f64, t9408: f64, t9411: f64, t1317: f64, t5567: f64, t2496: f64, t5571: f64, t5569: f64, t9597: f64, t123: f64, t1856: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13633 = t512 * t13632;
    let t13634 = 32.0_f64 * t9408;
    let t13635 = 80.0_f64 * t9411;
    let t13643 = 8.0_f64 * t1317 * t5567;
    let t13652 = t5571 * t2496;
    let t13654 = t1317 * t5569;
    let t13664 = 12.0_f64 * t9597;
    let t13665 = t1856 * t123;
    (t13633, t13634, t13635, t13643, t13652, t13654, t13664, t13665)
}
