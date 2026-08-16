//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2576/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2576(t300: f64, t71322: f64, t71664: f64, t71712: f64, t71752: f64, t71791: f64, t71828: f64, t71868: f64, t72041: f64, t18926: f64, t4869: f64, t1164: f64, t14960: f64, t6085: f64) -> (f64, f64, f64) {
    let t72045 = t300 * (t71322 + t71664 + t71712 + t71752 + t71791 + t71828 + t71868 + t72041);
    let t72047 = 0.17544670867903938621e1_f64 * t4869 * t18926;
    let t72050 = 0.35089341735807877242e1_f64 * t1164 * t14960 * t6085;
    (t72045, t72047, t72050)
}
