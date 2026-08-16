//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2472/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2472(t48089: f64, t221: f64, t9817: f64, t1320: f64, t13632: f64, t1317: f64, t13680: f64, t3860: f64, t5567: f64, t46971: f64, t3857: f64, t5569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48090 = 0.34697458558045176417e-2_f64 * t48089;
    let t48100 = t9817 * t221;
    let t48152 = t1320 * t13632;
    let t48157 = 24.0_f64 * t1317 * t13680;
    let t48158 = t3860 * t5567;
    let t48159 = 36.0_f64 * t48158;
    let t48224 = 480.0_f64 * t46971;
    let t48225 = t1317 * t13632;
    let t48226 = 12.0_f64 * t48225;
    let t48227 = t3857 * t5569;
    (t48090, t48100, t48152, t48157, t48159, t48224, t48226, t48227)
}
