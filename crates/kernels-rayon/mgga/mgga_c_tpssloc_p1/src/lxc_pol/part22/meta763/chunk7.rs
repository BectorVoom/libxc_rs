//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2577/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2577(t14858: f64, t6102: f64, t1157: f64, t1164: f64, t22228: f64, t1763: f64, t4700: f64, t64548: f64, t71255: f64, t71313: f64, t71315: f64, t71317: f64, t71319: f64, t72045: f64, t72047: f64, t72050: f64) -> (f64, f64, f64) {
    let t72052 = 0.17544670867903938621e1_f64 * t14858 * t6102;
    let t72058 = 0.14035736694323150897e2_f64 * t1164 * t22228 * t1157;
    let t72059 = -3.0_f64 * t1763 * t4700 * t64548 + t71255 + t71313 + t71315 + t71317 + t71319 + t72045 - t72047 + t72050 - t72052 + t72058;
    (t72052, t72058, t72059)
}
