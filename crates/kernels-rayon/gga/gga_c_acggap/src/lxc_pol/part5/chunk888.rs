//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 888/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk888(t1095: f64, t3101: f64, t372: f64, t384: f64, t398: f64, t1163: f64, t1165: f64, t3695: f64, t407: f64, t1160: f64, t12746: f64, t1167: f64) -> (f64, f64, f64, f64) {
    let t13161 = t384 * t398 * t1095 * t3101 * t372;
    let t13181 = t1163 * t1165 * t3695 * t407;
    let t13183 = t1160 * t12746;
    let t13184 = t13183 * t1167;
    (t13161, t13181, t13183, t13184)
}
