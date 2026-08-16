//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 907/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk907(t3111: f64, t3121: f64, t1068: f64, t3114: f64, t1072: f64, t3126: f64, t922: f64, t3124: f64, t839: f64, t130: f64, t972: f64, t1: f64, t136: f64, t14: f64, t195: f64, t3: f64, t721: f64) -> (f64, f64, f64, f64, f64) {
    let t13701 = t3111 * t3121;
    let t13703 = t1068 * t3114;
    let t13706 = t13703 * t1072 * t922 * t3126;
    let t13714 = t3124 * t1072 * t839 * t3126;
    let t13716 = t130 * t972;
    let t13726 = t13716 * t136 / t14 / t1 / t3 / t195 * t721 / 48.0_f64;
    (t13701, t13703, t13706, t13714, t13726)
}
