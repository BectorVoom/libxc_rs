//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1126/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1126(t6892: f64, t8950: f64, t1727: f64, t8891: f64, t3448: f64, t5384: f64, t16399: f64, t8996: f64, t6966: f64, t8968: f64, t17043: f64, t9000: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24219 = t6892 * t8950;
    let t24251 = t1727 * t8891;
    let t24259 = t5384 * t3448;
    let t24269 = t16399 * t8996;
    let t24272 = t6966 * t8968;
    let t24282 = t17043 * t9000;
    (t24219, t24251, t24259, t24269, t24272, t24282)
}
