//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2538/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2538(t10375: f64, t1612: f64, t1041: f64, t1539: f64, t248: f64, t42749: f64, t10661: f64, t1556: f64, t14363: f64, t300: f64, t14419: f64, t923: f64) -> (f64, f64, f64, f64, f64) {
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    let t48763 = t10661 * t1556;
    let t48766 = t300 * t14363;
    let t48771 = t14419 * t923;
    (t48670, t48674, t48763, t48766, t48771)
}
