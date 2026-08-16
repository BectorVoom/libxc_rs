//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2556/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2556(t300: f64, t51381: f64, t51411: f64, t51450: f64, t51493: f64, t51538: f64, t51617: f64, t51664: f64, t51789: f64, t15041: f64, t3411: f64, t11126: f64, t4884: f64) -> (f64, f64, f64) {
    let t51793 = t300 * (t51381 + t51411 + t51450 + t51493 + t51538 + t51617 + t51664 + t51789);
    let t51795 = 0.51947577317044391277e2_f64 * t3411 * t15041;
    let t51797 = 0.51947577317044391277e2_f64 * t11126 * t4884;
    (t51793, t51795, t51797)
}
