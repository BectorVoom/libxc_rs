//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1415/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1415(t2927: f64, t30599: f64, t1539: f64, t1605: f64, t2881: f64, t2868: f64, t1149: f64, t1167: f64, t1169: f64, t22639: f64, t22954: f64, t26727: f64, t26886: f64, t2876: f64, t30571: f64, t30574: f64, t30578: f64, t30586: f64, t30596: f64, t4524: f64, t518: f64, t9636: f64) -> (f64, f64, f64, f64, f64) {
    let t30600 = t2927 * t30599;
    let t30603 = t1605 * t1539;
    let t30604 = t2881 * t30603;
    let t30607 = t2868 * t30599;
    let t30610 = 3024.0_f64 * t518 * t22639 * t4524 * t2876 + 256.0_f64 / 9.0_f64 * t22954 * t30571 + 12800.0_f64 / 729.0_f64 * t1169 * t30574 * t30578 + 12800.0_f64 / 729.0_f64 * t1167 * t30574 * t30578 + 6400.0_f64 / 243.0_f64 * t1167 * t26727 * t30586 + 6400.0_f64 / 243.0_f64 * t1169 * t26727 * t30586 + 6400.0_f64 / 81.0_f64 * t1149 * t26727 * t30586 - 3200.0_f64 / 81.0_f64 * t26886 * t30596 + 3200.0_f64 / 81.0_f64 * t30600 * t9636 - 1600.0_f64 / 27.0_f64 * t30604 * t9636 + 8000.0_f64 / 27.0_f64 * t30607 * t9636;
    (t30600, t30603, t30604, t30607, t30610)
}
