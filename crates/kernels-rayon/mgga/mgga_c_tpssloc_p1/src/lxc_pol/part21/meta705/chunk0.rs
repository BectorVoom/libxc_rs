//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2537/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2537(t3067: f64, t353: f64, t373: f64, t383: f64, t1021: f64, t820: f64, t10482: f64, t1615: f64, t1041: f64, t13969: f64, t14142: f64, t14179: f64) -> (f64, f64, f64, f64, f64) {
    let t48607 = t353 * t383 * t3067 * t373;
    let t48611 = t820 * t1021;
    let t48612 = t1615 * t10482;
    let t48626 = t1041 * t13969 * t14142;
    let t48629 = t1041 * t13969 * t14179;
    (t48607, t48611, t48612, t48626, t48629)
}
