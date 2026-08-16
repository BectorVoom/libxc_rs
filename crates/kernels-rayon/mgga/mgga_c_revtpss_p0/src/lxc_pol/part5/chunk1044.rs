//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1044/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1044(t9395: f64, t2626: f64, t5571: f64, t1856: f64, t2608: f64, t512: f64, t9408: f64, t9411: f64, t1317: f64, t5567: f64, t2496: f64, t9597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13623 = 4.0_f64 * t9395;
    let t13630 = t5571 * t2626;
    let t13632 = t1856 * t2608;
    let t13633 = t512 * t13632;
    let t13634 = 32.0_f64 * t9408;
    let t13635 = 80.0_f64 * t9411;
    let t13643 = 8.0_f64 * t1317 * t5567;
    let t13652 = t5571 * t2496;
    let t13664 = 12.0_f64 * t9597;
    (t13623, t13630, t13633, t13634, t13635, t13643, t13652, t13664)
}
