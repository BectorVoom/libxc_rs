//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3134/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134(t1174: f64, t15293: f64, t15320: f64, t3447: f64, t3449: f64, t457: f64, t460: f64, t4733: f64, t4908: f64, t4919: f64, t52122: f64, t52124: f64, t52170: f64, t64851: f64, t64858: f64, t64870: f64, t64874: f64, t64878: f64, t64881: f64, t7319: f64, t974: f64) -> f64 {
    let t64883 = -0.98765432098765432094e-3_f64 * t52122 - 0.16460905349794238682e-2_f64 * t52124 - 0.16666666666666666666e-2_f64 * t1174 * t974 * t457 * t64851 * t460 - 0.55555555555555555554e-3_f64 * t64858 + 0.55555555555555555554e-3_f64 * t3447 * t4919 * t52170 + 0.11111111111111111111e-2_f64 * t3447 * t4919 * t7319 * t4733 + 0.22222222222222222222e-2_f64 * t3447 * t15320 * t15293 + 0.22222222222222222222e-2_f64 * t3447 * t3449 * t64870 - 0.66666666666666666665e-2_f64 * t3447 * t4908 * t64874 - 0.18106995884773662551e-2_f64 * t64878 + 0.18518518518518518518e-3_f64 * t64881;
    t64883
}
