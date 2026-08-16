//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 741/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk741(t2127: f64, t7122: f64, t2126: f64, t6927: f64, t115: f64, t138: f64, t5: f64, t6932: f64, t6937: f64, t2124: f64, t2168: f64, t3467: f64, t3501: f64, t6782: f64, t6787: f64, t6792: f64, t6928: f64, t7111: f64, t7113: f64, t7116: f64, t7119: f64) -> (f64, f64, f64, f64, f64) {
    let t7123 = t7122 * t2127;
    let t7125 = t2126 * t6927;
    let t7128 = t138 * t115;
    let t7129 = t7128 * t5;
    let t7130 = t7129 * t6932;
    let t7133 = t2126 * t6937;
    let t7136 = 0.18137053605011111023e0_f64 * t2168 * t6928 + 0.18137053605011111023e0_f64 * t2168 * t6782 - 0.5441116081503333307e0_f64 * t3501 * t6787 + 0.13602790203758333267e0_f64 * t3501 * t6792 - 0.16927916698010370288e1_f64 * t7111 + 0.52158968938732547127e0_f64 * t2124 * t7113 - 0.26079484469366273564e0_f64 * t2124 * t7116 + 0.52158968938732547127e0_f64 * t3467 * t7119 - 0.24340852171408521993e1_f64 * t7123 + 0.52158968938732547127e0_f64 * t2124 * t7125 - 0.15647690681619764138e1_f64 * t2124 * t7130 + 0.52158968938732547127e0_f64 * t2124 * t7133;
    (t7125, t7129, t7130, t7133, t7136)
}
