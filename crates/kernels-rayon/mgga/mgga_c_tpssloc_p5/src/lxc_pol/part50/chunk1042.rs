//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1042/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1042(t1030: f64, t383: f64, t353: f64, t1036: f64, t8388: f64, t362: f64, t368: f64, t372: f64, t354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30832 = t383 * t1030;
    let t30833 = t353 * t30832;
    let t30837 = t8388 * t1036 / 2304.0_f64;
    let t30838 = t362 * t368;
    let t30839 = t30838 * t372;
    let t30840 = t354 * t30839;
    (t30832, t30833, t30837, t30838, t30839, t30840)
}
